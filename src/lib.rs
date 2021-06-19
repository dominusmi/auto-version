use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{ItemFn};

/// This attribute macro which prints the value of CARGO_PKG_VERSION and exits with status code 0
/// if the command line arguments include either `-v` or `--version`
///
/// In the case where the code is not compiled with cargo, the version will be replaced with the message
/// "`auto_version` macro only works for projects compiled with cargo".
///
/// Example:
/// ```rust
/// #[auto_version]
/// fn main() {
///     // executed code
/// }
/// ```
/// Then, when this binary is called with `binary -v` or `binary --version`, it will output the `Cargo.toml`
/// version without any specific formatting:
/// ```shell
/// $ ./binary -v
/// $ 0.1.0
/// ```

#[proc_macro_attribute]
pub fn auto_version(_: TokenStream, input: TokenStream) -> TokenStream {

    let mut item: syn::Item = syn::parse(input).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn")
    };

    let version_tokens = quote! {
        {
            // use this inner block so that the local imports don't affect the user's code
            use std::ffi::{OsString, OsStr};
            use std::process::exit;
            let args: Vec<OsString> = std::env::args_os().collect();
            let v = OsStr::new("-v");
            let version = OsStr::new("--version");
            for arg in args.iter() {
                if arg == v || arg == version {
                    match option_env!("CARGO_PKG_VERSION") {
                        Some(version) => println!("{}", version),
                        None => println!("`auto_version` macro only works for projects compiled with cargo")
                    }
                    exit(0);
                }
            }
        }
    };

    let ItemFn { attrs, vis, sig, block } = fn_item;
    let stmts = &block.stmts;
    let stream = quote! {
        #(#attrs)* #vis #sig {
            #version_tokens
            #(#stmts)*
        }
    };
    stream.into()
}