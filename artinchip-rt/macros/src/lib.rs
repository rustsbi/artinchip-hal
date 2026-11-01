//! ArtInChip ROM and application runtime procedural macros.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    FnArg, ItemFn, LitStr, ReturnType, Type, Visibility, parse, parse_macro_input, spanned::Spanned,
};

/// Pre-Boot Program (PBP) entry point.
///
/// ```ignore
/// #[pbp_entry]
/// fn pbp_main(boot_param: BootParam, private_data: &[u8]) { ... }
/// ```
#[proc_macro_attribute]
pub fn pbp_entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "`#[pbp_entry]` accepts no arguments")
            .to_compile_error()
            .into();
    }

    if let Err(err) = validate_boot_abi_inputs(&f, "`#[pbp_entry]`") {
        return err.into();
    }

    let valid_return = match &f.sig.output {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => matches!(&**ty, Type::Never(_)),
    };
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && valid_return;

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[pbp_entry]` function must have signature `[unsafe] fn pbp_main(boot_param: BootParam, private_data: &[u8]) [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let safety = f.sig.safety;
    let inputs = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let ident = f.sig.ident;

    quote!(
        ::artinchip_rt::__rt_pbp_startup!();

        #[unsafe(export_name = "pbp_main")]
        #(#attrs)*
        pub unsafe extern "C" fn #ident(boot_param: u32, priv_addr: *const u8, priv_len: u32) #ret {
            let private_data = if priv_addr.is_null() || priv_len == 0 {
                &[]
            } else {
                unsafe { ::core::slice::from_raw_parts(priv_addr, priv_len as usize) }
            };
            let boot_param = ::artinchip_rt::core::boot_rom::BootParam::from_raw(boot_param);
            #[allow(unused_unsafe)]
            unsafe { __artinchip_rt__pbp_main(boot_param, private_data) }
        }

        #[allow(non_snake_case)]
        #[inline]
        #(#attrs)*
        #safety fn __artinchip_rt__pbp_main(#inputs) #ret {
            #(#stmts)*
        }
    )
    .into()
}

/// Application image entry point (`AIC` loader resource).
///
/// Dispatches by function signature:
/// - `async fn app_main()`: starts the built-in Embassy executor (with `CPU_TIMER`
///   bound automatically) and polls the entry task.
/// - `fn app_main(boot_param: BootParam, private_data: &[u8]) [-> !]`: runs
///   synchronously with the BootROM/loader arguments (e.g. `app-bootloader`).
///
/// The image name is read from `env!("AIC_APP_NAME")` when invoked as `#[app_entry]`,
/// or given explicitly via `#[app_entry("blinky")]`.
#[proc_macro_attribute]
pub fn app_entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let name_expr = match parse_app_name(&args) {
        Ok(expr) => expr,
        Err(err) => return err.into(),
    };
    let markers = app_markers(name_expr);

    if f.sig.asyncness.is_some() {
        expand_async_app_entry(f, markers)
    } else {
        expand_sync_app_entry(f, markers)
    }
}

/// Bootloader entry (synchronous, bootloader partition only).
///
/// This entry is fixed to the bootloader image in the `bootloader` partition.
/// It takes no name argument: the bootloader's AIC name and partition come from
/// the board preset, not from the entry macro. Boot-chain images must use this
/// entry instead of [`app_entry`].
///
/// ```ignore
/// #[boot_entry]
/// fn app_main(boot_param: BootParam, private_data: &[u8]) -> ! { ... }
/// ```
#[proc_macro_attribute]
pub fn boot_entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "`#[boot_entry]` accepts no arguments")
            .to_compile_error()
            .into();
    }
    let markers = app_markers(quote!("bootloader"));

    if let Err(err) = validate_boot_abi_inputs(&f, "`#[boot_entry]`") {
        return err.into();
    }
    let valid_return = match &f.sig.output {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => matches!(&**ty, Type::Never(_)),
    };
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && valid_return;
    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[boot_entry]` function must have signature `fn app_main(boot_param: BootParam, private_data: &[u8]) [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let safety = f.sig.safety;
    let inputs = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let ident = f.sig.ident;

    quote!(
        ::artinchip_rt::__rt_boot_startup!();

        #[unsafe(export_name = "app_main")]
        #(#attrs)*
        pub unsafe extern "C" fn #ident(boot_param: u32, priv_addr: *const u8, priv_len: u32) #ret {
            let private_data = if priv_addr.is_null() || priv_len == 0 {
                &[]
            } else {
                unsafe { ::core::slice::from_raw_parts(priv_addr, priv_len as usize) }
            };
            let boot_param = ::artinchip_rt::core::boot_rom::BootParam::from_raw(boot_param);
            #[allow(unused_unsafe)]
            unsafe { __artinchip_rt__app_main(boot_param, private_data) }
        }

        #[allow(non_snake_case)]
        #[inline]
        #(#attrs)*
        #safety fn __artinchip_rt__app_main(#inputs) #ret {
            #(#stmts)*
        }

        #markers
    )
    .into()
}

fn expand_async_app_entry(f: ItemFn, markers: proc_macro2::TokenStream) -> TokenStream {
    if let Some(arg) = f.sig.inputs.first() {
        return parse::Error::new(
            arg.span(),
            "async `#[app_entry]` function takes no arguments; use a synchronous `fn(BootParam, &[u8])` entry if BootROM arguments are required",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && matches!(f.sig.safety, syn::Safety::Default)
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Default);
    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "async `#[app_entry]` function must have signature `async fn app_main()`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let stmts = f.block.stmts;
    let ident = f.sig.ident;

    quote!(
        ::artinchip_rt::__rt_app_startup!();

        #[unsafe(export_name = "app_main")]
        #(#attrs)*
        pub unsafe extern "C" fn #ident(_boot_param: u32, _priv_addr: *const u8, _priv_len: u32) -> ! {
            #[::artinchip_rt::embassy_executor::task(
                embassy_executor = ::artinchip_rt::embassy_executor
            )]
            #[allow(clippy::future_not_send)]
            async fn __artinchip_rt__app_main() {
                #(#stmts)*
            }

            let task = __artinchip_rt__app_main()
                .expect("application entry task pool is exhausted");
            ::artinchip_rt::app::run_task(task)
        }

        #markers
    )
    .into()
}

fn expand_sync_app_entry(f: ItemFn, markers: proc_macro2::TokenStream) -> TokenStream {
    if let Err(err) = validate_boot_abi_inputs(&f, "synchronous `#[app_entry]`") {
        return err.into();
    }

    let valid_return = match &f.sig.output {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => matches!(&**ty, Type::Never(_)),
    };
    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && valid_return;
    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "synchronous `#[app_entry]` function must have signature `[unsafe] fn app_main(boot_param: BootParam, private_data: &[u8]) [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let safety = f.sig.safety;
    let inputs = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let ident = f.sig.ident;

    quote!(
        ::artinchip_rt::__rt_app_startup!();

        #[unsafe(export_name = "app_main")]
        #(#attrs)*
        pub unsafe extern "C" fn #ident(boot_param: u32, priv_addr: *const u8, priv_len: u32) #ret {
            let private_data = if priv_addr.is_null() || priv_len == 0 {
                &[]
            } else {
                unsafe { ::core::slice::from_raw_parts(priv_addr, priv_len as usize) }
            };
            let boot_param = ::artinchip_rt::core::boot_rom::BootParam::from_raw(boot_param);
            #[allow(unused_unsafe)]
            unsafe { __artinchip_rt__app_main(boot_param, private_data) }
        }

        #[allow(non_snake_case)]
        #[inline]
        #(#attrs)*
        #safety fn __artinchip_rt__app_main(#inputs) #ret {
            #(#stmts)*
        }

        #markers
    )
    .into()
}

fn validate_boot_abi_inputs(f: &ItemFn, macro_label: &str) -> Result<(), proc_macro2::TokenStream> {
    if f.sig.inputs.len() != 2 {
        return Err(parse::Error::new(
            f.sig
                .inputs
                .last()
                .map_or_else(Span::call_site, Spanned::span),
            format!(
                "{macro_label} function must take `(boot_param: BootParam, private_data: &[u8])`"
            ),
        )
        .to_compile_error());
    }

    for (index, arg) in f.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(_) => {
                return Err(
                    parse::Error::new(arg.span(), "entry function cannot take `self`")
                        .to_compile_error(),
                );
            }
            FnArg::Typed(t) => match (index, &*t.ty) {
                (0, Type::Path(_)) => {}
                (1, Type::Reference(p)) if matches!(&*p.elem, Type::Slice(_)) => {}
                (0, _) => {
                    return Err(parse::Error::new(
                        t.ty.span(),
                        "first parameter must be `BootParam`",
                    )
                    .to_compile_error());
                }
                _ => {
                    return Err(
                        parse::Error::new(t.ty.span(), "second parameter must be `&[u8]`")
                            .to_compile_error(),
                    );
                }
            },
        }
    }
    Ok(())
}

fn parse_app_name(
    args: &TokenStream,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    if args.is_empty() {
        return Ok(quote!(::core::env!("AIC_APP_NAME")));
    }

    let name = syn::parse::<LitStr>(args.clone()).map_err(|_| {
        parse::Error::new(
            Span::call_site(),
            "`#[app_entry]` expects either no arguments (`AIC_APP_NAME` env) or a string literal",
        )
        .to_compile_error()
    })?;

    let value = name.value();
    if value.is_empty()
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(
            parse::Error::new(name.span(), "application name must match [A-Za-z0-9_-]+")
                .to_compile_error(),
        );
    }

    Ok(quote!(#name))
}

fn app_markers(name_expr: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote!(
        #[doc(hidden)]
        const __AIC_APP_NAME: &str = #name_expr;

        const _: () = {
            let bytes = __AIC_APP_NAME.as_bytes();
            assert!(!bytes.is_empty(), "application name must not be empty");
            let mut i = 0;
            while i < bytes.len() {
                let b = bytes[i];
                assert!(
                    b.is_ascii_alphanumeric() || b == b'_' || b == b'-',
                    "application name must match [A-Za-z0-9_-]+"
                );
                i += 1;
            }
        };

        /// Null-terminated application name stored in `.app.name` for image tooling.
        #[doc(hidden)]
        #[unsafe(link_section = ".app.name")]
        #[used]
        pub static APP_ENTRY_NAME: [u8; __AIC_APP_NAME.len() + 1] = {
            let bytes = __AIC_APP_NAME.as_bytes();
            let mut out = [0u8; __AIC_APP_NAME.len() + 1];
            let mut i = 0;
            while i < bytes.len() {
                out[i] = bytes[i];
                i += 1;
            }
            out
        };
    )
}
