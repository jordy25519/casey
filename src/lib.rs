mod traits;
use proc_macro::{Ident, Span, TokenStream, TokenTree};
use traits::{PascalCaseExt, ShoutySnakeCaseExt, SnakeCaseExt};

macro_rules! transform_idents {
    ($stream:ident, $transform:ident) => {
        $stream
            .into_iter()
            .map(|t| match t {
                TokenTree::Ident(ident) => {
                    Ident::new(&ident.to_string().$transform(), Span::call_site()).into()
                }
                _ => t,
            })
            .collect()
    };
}

/// Expands idents in the input stream as UPPERCASE
#[proc_macro]
pub fn upper(stream: TokenStream) -> TokenStream {
    transform_idents!(stream, to_uppercase)
}

/// Expands idents in the input stream as lowercase
#[proc_macro]
pub fn lower(stream: TokenStream) -> TokenStream {
    transform_idents!(stream, to_lowercase)
}

/// Expands idents in the input stream as snake_case
/// e.g. `HelloWorld` -> `hello_world`
#[proc_macro]
pub fn snake(stream: TokenStream) -> TokenStream {
    transform_idents!(stream, to_snake_case)
}

/// Expands idents in the input stream as PascalCase
/// e.g. `helloWorld` -> `HelloWorld`
#[proc_macro]
pub fn pascal(stream: TokenStream) -> TokenStream {
    transform_idents!(stream, to_pascal_case)
}

/// Expands idents in the input stream as SHOUTY_CASE
/// e.g. `HelloWorld` -> `HELLO_WORLD`
#[proc_macro]
pub fn shouty(stream: TokenStream) -> TokenStream {
    transform_idents!(stream, to_shouty_snake_case)
}
