extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

extern crate self as blf_lib_derive;

use proc_macro::TokenStream;

mod helpers;
mod macros;

#[proc_macro_derive(BlfChunk, attributes(Header, Size))]
pub fn blf_chunk(input: TokenStream) -> TokenStream {
    macros::blf_chunk::blf_chunk_macro(input)
}

#[proc_macro_derive(TitleAndBuild, attributes(Title, Build))]
pub fn title_and_build(input: TokenStream) -> TokenStream {
    macros::title_and_build::title_and_build_macro(input)
}

#[proc_macro_derive(BlfFile)]
pub fn blf_file(input: TokenStream) -> TokenStream {
    macros::blf_file::blf_file_macro(input)
}

#[proc_macro_derive(TestSize, attributes(Size))]
pub fn test_size(input: TokenStream) -> TokenStream {
    macros::test_size::test_size_macro(input)
}
