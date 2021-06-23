#![forbid(unsafe_code)]

use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn safe(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    //Choose a good safe word.
    //This is spanish for "a safe"
    let safe_word: String = "un safe".chars().filter(|s| !s.is_whitespace()).collect();

    //Now, add our safe word to the ouput
    let safe_ident = Ident::new(&safe_word, Span::call_site());
    output.extend(Some(TokenTree::from(safe_ident)));

    //then, stick the scary code into a pair of braces
    let expr = Group::new(Delimiter::Brace, input);
    output.extend(Some(TokenTree::from(expr)));

    output
}
