mod traits;
use proc_macro::{Group, Ident, Span, TokenStream, TokenTree};
use traits::{PascalCaseExt, ShoutySnakeCaseExt, SnakeCaseExt};

/// Apply a string transformation (`transform`) to the input `Ident`-
/// However, it will not apply the transform to rust keywords.
fn transform_non_keyword_ident<Transform>(ident: &Ident, transform: &Transform) -> Ident
where
    Transform: Fn(String) -> String,
{
    let ident_value = ident.to_string();
    let is_keyword = syn::parse_str::<syn::Ident>(ident_value.as_str()).is_err();
    if is_keyword {
        ident.clone()
    } else {
        Ident::new(transform(ident_value).as_str(), Span::call_site())
    }
}

fn transform_idents_in_stream2<Transform>(stream: TokenStream, transform: &Transform) -> TokenStream
where
    Transform: Fn(String) -> String,
{
    let mut transformed = TokenStream::new();
    let mut attr_macro_hit = false;
    for tt_in in stream {
        let tt_out = match tt_in {
            TokenTree::Punct(punct) => {
                attr_macro_hit = punct.as_char() == '#';
                punct.into()
            }
            TokenTree::Literal(l) => {
                attr_macro_hit = attr_macro_hit && l.to_string() == "[";
                l.into()
            }
            TokenTree::Ident(ref ident) => transform_non_keyword_ident(ident, transform).into(),
            // find all idents in `TokenGroup` apply and reconstruct the group
            TokenTree::Group(ref group) => TokenTree::Group(Group::new(
                group.delimiter(),
                group
                    .stream()
                    .into_iter()
                    .map(|group_token_tree| {
                        if let TokenTree::Ident(ref ident) = group_token_tree {
                            if attr_macro_hit {
                                attr_macro_hit = false;
                                TokenTree::Ident(ident.clone())
                            } else {
                                transform_non_keyword_ident(ident, transform).into()
                            }
                        } else {
                            group_token_tree
                        }
                    })
                    .collect::<TokenStream>(),
            )),
        };
        transformed.extend([tt_out]);
    }
    transformed
}

/// Expands idents in the input stream as UPPERCASE
#[proc_macro]
pub fn upper(stream: TokenStream) -> TokenStream {
    transform_idents_in_stream2(stream, &|s: String| s.to_uppercase())
}

/// Expands idents in the input stream as lowercase
#[proc_macro]
pub fn lower(stream: TokenStream) -> TokenStream {
    transform_idents_in_stream2(stream, &|s: String| s.to_lowercase())
}

/// Expands idents in the input stream as snake_case
/// e.g. `HelloWorld` -> `hello_world`
#[proc_macro]
pub fn snake(stream: TokenStream) -> TokenStream {
    transform_idents_in_stream2(stream, &|s: String| s.to_snake_case())
}

/// Expands idents in the input stream as PascalCase
/// e.g. `helloWorld` -> `HelloWorld`
#[proc_macro]
pub fn pascal(stream: TokenStream) -> TokenStream {
    transform_idents_in_stream2(stream, &|s: String| s.to_pascal_case())
}

/// Expands idents in the input stream as SHOUTY_CASE
/// e.g. `HelloWorld` -> `HELLO_WORLD`
#[proc_macro]
pub fn shouty(stream: TokenStream) -> TokenStream {
    transform_idents_in_stream2(stream, &|s: String| s.to_shouty_snake_case())
}
