//! Prevent your panics from unwinding past FFI boundaries with this one simple trick!
//!
//! Intended to be used in place of [`#[unwind(aborts)]`][unwind_aborts_upstream]
//! until it is stabilized.
//!
//! ## Usage
//!
//! Add this to your `[dependencies]` in `Cargo.toml`:
//!
//! ```toml
//! unwind_aborts = "0.1.0"
//! ```
//!
//! Annotate your functions with `#[unwind_aborts]` to catch stack unwinding and
//! abort the process instead:
//!
//! ```rust
//! use unwind_aborts::unwind_aborts;
//!
//! #[unwind_aborts]
//! pub extern fn foo() {
//!     panic!("this is safe");
//! }
//! ```
//!
//! The example above is equivalent to:
//!
//! ```rust
//! pub extern fn foo() {
//!     let result = std::panic::catch_unwind(|| {
//!         panic!("this is safe");
//!     });
//!     match result {
//!         Ok(x) => x,
//!         Err(_) => std::process::abort(),
//!     }
//! }
//! ```
//!
//! [unwind_aborts_upstream]: https://github.com/rust-lang/rust/issues/58760

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
