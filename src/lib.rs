use proc_macro2::{Delimiter, TokenStream, TokenTree};

#[proc_macro]
pub fn nvim(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    proc_macro::TokenStream::from(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
