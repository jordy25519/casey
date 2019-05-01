extern crate proc_macro;
use proc_macro::{Ident, Span, TokenStream, TokenTree};

///
/// Expands an input `ident` as uppercase
///
/// # Example
/// ```
/// use casey::upper;
/// upper!(HelloWorld) // HELLOWORLD
/// ```
#[proc_macro]
pub fn upper(input: TokenStream) -> TokenStream {
    let ident = Ident::new(&input.to_string().to_uppercase(), Span::call_site());
    TokenStream::from(TokenTree::Ident(ident))
}

///
/// Expands an input `ident` as lowercase
///
/// # Example
/// ```
/// use casey::lower;
/// lower!(HelloWorld) // helloworld
/// ```
#[proc_macro]
pub fn lower(input: TokenStream) -> TokenStream {
    let ident = Ident::new(&input.to_string().to_lowercase(), Span::call_site());
    TokenStream::from(TokenTree::Ident(ident))
}

///
/// Expands an input `ident` as snakecase
///
/// e.g. `HelloWorld` -> `hello_world`
/// # Example
/// ```
/// use casey::snake;
/// snake!(HelloWorld) // hello_world
/// ```
#[proc_macro]
pub fn snake(input: TokenStream) -> TokenStream {
    let raw_ident = &input.to_string();
    let mut s = String::new();
    for (i, c) in raw_ident.chars().enumerate() {
        if c.is_uppercase() || c.is_numeric() {
            if i > 0 {
                s.push('_');
            }
            s.push(c.to_lowercase().to_string().chars().next().unwrap());
        } else {
            s.push(c)
        }
    }
    let ident = Ident::new(&s, Span::call_site());
    TokenStream::from(TokenTree::Ident(ident))
}
