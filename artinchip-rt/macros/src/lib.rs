//! Artinchip ROM runtime procedural macros.

use proc_macro2::Span;
use quote::quote;
use syn::{
    FnArg, ItemFn, ReturnType, Type, Visibility, parse, parse_macro_input, spanned::Spanned,
};

use proc_macro::TokenStream;

/// Pre-Boot Program (PBP) entry.
///
/// In Rust, PBP entry function should follow this convention listed below:
///
/// ```rust
/// #[pbp_entry]
/// [unsafe] fn pbp_main(boot_param: u32, private_data: &[u8])
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
            "`#[pbp_entry]` function must have signature `[unsafe] fn pbp_main(boot_param: u32, private_data: &[u8])`",
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
    let unsafety = f.sig.unsafety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;
    let ident = f.sig.ident;

    quote!(
        #[unsafe(export_name = "pbp_main")]
        #(#attrs)*
        pub extern "C" fn #ident(boot_param: u32, priv_addr: *const u8, priv_len: u32) #ret {
            let private_data = unsafe { core::slice::from_raw_parts(priv_addr, priv_len as usize) };
            unsafe { __artinchip_rt__pbp_main(boot_param, private_data ) }
        }
        #[allow(non_snake_case)]
        #[inline]
        #(#attrs)*
        #unsafety fn __artinchip_rt__pbp_main(#args) #ret {
            #(#stmts)*
        }
    )
    .into()
}
