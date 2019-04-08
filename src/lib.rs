#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::{Span, TokenStream};


#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        Span::call_site()
            .warning("Input is empty!")
            .emit();
    }

    // Just return the exact input
    input
}
