//! ArtInChip ROM runtime procedural macros.

use proc_macro2::Span;
use quote::quote;
use syn::{
    FnArg, ItemFn, LitStr, ReturnType, Type, Visibility, parse, parse_macro_input, spanned::Spanned,
};

use proc_macro::TokenStream;

/// Pre-Boot Program (PBP) entry.
///
/// The first parameter must be `BootParam` from `artinchip_rt::core::boot_rom`:
///
/// ```rust
/// use artinchip_rt::core::boot_rom::BootParam;
///
/// #[pbp_entry]
/// fn pbp_main(boot_param: BootParam, private_data: &[u8])
/// ```
#[proc_macro_attribute]
pub fn pbp_entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function arguments
    if f.sig.inputs.len() != 2 {
        return parse::Error::new(
            f.sig.inputs.last().unwrap().span(),
            "`#[pbp_entry]` function should include exactly two parameters",
        )
        .to_compile_error()
        .into();
    }

    let arg0_boot_param = &f.sig.inputs[0];
    let arg1_private_data = &f.sig.inputs[1];

    match arg0_boot_param {
        FnArg::Receiver(_) => {
            return parse::Error::new(
                arg0_boot_param.span(),
                "artinchip-rt-macros: invalid argument",
            )
            .to_compile_error()
            .into();
        }
        FnArg::Typed(t) => {
            if let Type::Path(_p) = &*t.ty {
                // empty
            } else {
                return parse::Error::new(
                    t.ty.span(),
                    "artinchip-rt-macros: argument type must be a path",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    match arg1_private_data {
        FnArg::Receiver(_) => {
            return parse::Error::new(
                arg1_private_data.span(),
                "artinchip-rt-macros: invalid argument",
            )
            .to_compile_error()
            .into();
        }
        FnArg::Typed(t) => {
            if let Type::Reference(p) = &*t.ty
                && let Type::Slice(_s) = &*p.elem
            {
                // empty
            } else {
                return parse::Error::new(
                    t.ty.span(),
                    "artinchip-rt-macros: argument type must be a reference to a slice",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Default);

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[pbp_entry]` function must have signature `[unsafe] fn pbp_main(boot_param: BootParam, private_data: &[u8])`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs;
    let safety = f.sig.safety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let ident = f.sig.ident;

    quote!(
        // The wrapper is the symbol the Boot/Rom startup jumps to, so its contract
        // (the pointer and length come from the boot loader, not from Rust) cannot
        // be checked. `unsafe` is how that is stated, and it is also what keeps
        // `clippy::not_unsafe_ptr_arg_deref` quiet - the alternative, an `allow`,
        // would hide the contract instead of documenting it.
        #[unsafe(export_name = "pbp_main")]
        #(#attrs)*
        pub unsafe extern "C" fn #ident(boot_param: u32, priv_addr: *const u8, priv_len: u32) #ret {
            // Images without a DATA2/private resource pass a null pointer.
            // `from_raw_parts(null, 0)` is still invalid Rust, so represent
            // that case with the canonical empty slice.
            let private_data = if priv_addr.is_null() || priv_len == 0 {
                &[]
            } else {
                unsafe { core::slice::from_raw_parts(priv_addr, priv_len as usize) }
            };
            let boot_param = ::artinchip_rt::core::boot_rom::BootParam::from_raw(boot_param);
            unsafe { __artinchip_rt__pbp_main(boot_param, private_data ) }
        }
        #[allow(non_snake_case)]
        #[inline]
        #(#attrs)*
        #safety fn __artinchip_rt__pbp_main(#args) #ret {
            #(#stmts)*
        }
    )
    .into()
}

/// Application image entry.
///
/// Unlike [`pbp_entry`], an application is a normal image loaded by the boot
/// chain (PBP/bootloader), so it has its own startup symbol. The name argument
/// identifies the app and is embedded as a marker symbol plus an `.app.name`
/// section so different apps can be told apart in the image/tooling:
///
/// ```rust
/// use artinchip_rt::core::boot_rom::BootParam;
///
/// #[app_entry("app-bootloader")]
/// fn app_main(boot_param: BootParam, private_data: &[u8])
/// ```
#[proc_macro_attribute]
pub fn app_entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let name = match syn::parse::<LitStr>(args) {
        Ok(lit) => lit,
        Err(_) => {
            return parse::Error::new(
                Span::call_site(),
                "`#[app_entry(\"name\")]` requires the application name as a string literal",
            )
            .to_compile_error()
            .into();
        }
    };
    let app_name = name.value();
    if app_name.is_empty()
        || !app_name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return parse::Error::new(name.span(), "application name must be [A-Za-z0-9_-]+")
            .to_compile_error()
            .into();
    }

    // Same ABI as the loader chain: (boot_param, private_data).
    if f.sig.inputs.len() != 2 {
        return parse::Error::new(
            f.sig
                .inputs
                .last()
                .map_or_else(Span::call_site, Spanned::span),
            "`#[app_entry]` function should include exactly two parameters",
        )
        .to_compile_error()
        .into();
    }
    for (index, arg) in f.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(_) => {
                return parse::Error::new(arg.span(), "artinchip-rt-macros: invalid argument")
                    .to_compile_error()
                    .into();
            }
            FnArg::Typed(t) => match (index, &*t.ty) {
                (0, Type::Path(_)) => {}
                (1, Type::Reference(p)) if matches!(&*p.elem, Type::Slice(_)) => {}
                (0, _) => {
                    return parse::Error::new(
                        t.ty.span(),
                        "artinchip-rt-macros: first argument must be a path type",
                    )
                    .to_compile_error()
                    .into();
                }
                _ => {
                    return parse::Error::new(
                        t.ty.span(),
                        "artinchip-rt-macros: second argument must be a reference to a slice",
                    )
                    .to_compile_error()
                    .into();
                }
            },
        }
    }

    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Default);
    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[app_entry]` function must have signature `fn app_main(boot_param: BootParam, private_data: &[u8])`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let safety = f.sig.safety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let ident = f.sig.ident;

    // Per-app marker symbols are plain identifiers, names are already
    // restricted to [A-Za-z0-9_-].
    let marker = format!("__app_entry_{}", app_name.replace('-', "_"));
    let name_len = app_name.len() + 1;
    let name_bytes = app_name.as_bytes();

    quote!(
        // See `pbp_entry`: the contract is the boot chain's, not Rust's.
        #[unsafe(export_name = "app_main")]
        #(#attrs)*
        pub unsafe extern "C" fn #ident(boot_param: u32, priv_addr: *const u8, priv_len: u32) #ret {
            let private_data = if priv_addr.is_null() || priv_len == 0 {
                &[]
            } else {
                unsafe { core::slice::from_raw_parts(priv_addr, priv_len as usize) }
            };
            let boot_param = ::artinchip_rt::core::boot_rom::BootParam::from_raw(boot_param);
            unsafe { __artinchip_rt__app_main(boot_param, private_data) }
        }
        #[allow(non_snake_case)]
        #[inline]
        #(#attrs)*
        #safety fn __artinchip_rt__app_main(#args) #ret {
            #(#stmts)*
        }

        /// Application name marker (unique symbol per app).
        #[doc(hidden)]
        #[unsafe(export_name = #marker)]
        #[used]
        pub static APP_ENTRY_MARKER: u8 = 0;

        /// Application name bytes, kept in `.app.name` for image tooling.
        #[doc(hidden)]
        #[unsafe(link_section = ".app.name")]
        #[used]
        pub static APP_ENTRY_NAME: [u8; #name_len] = [#(#name_bytes,)* 0];
    )
    .into()
}
