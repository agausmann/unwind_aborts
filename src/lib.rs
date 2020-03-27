use proc_macro2::TokenStream;

use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn unwind_aborts(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match unwind_aborts_inner(args.into(), item.into()) {
        Ok(out) => out.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn unwind_aborts_inner(args: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
    if !args.is_empty() {
        return Err(syn::Error::new_spanned(
            args,
            "arguments are not allowed for `unwind_aborts` attributes",
        ));
    }
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse2(item)?;
    Ok(quote! {
        #(#attrs)* #vis #sig {
            match std::panic::catch_unwind(|| #block) {
                Ok(v) => v,
                Err(_) => std::process::abort(),
            }
        }
    })
}
