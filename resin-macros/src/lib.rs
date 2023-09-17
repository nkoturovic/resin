extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
pub fn resin_model(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input as an item (expecting a struct definition).
    let input = parse_macro_input!(input as ItemStruct);
    let input_struct_name = &input.ident;
    let input_fields_iter = input.fields.iter();
    let model_struct_name = format_ident!("{}Model", input_struct_name);

    TokenStream::from(quote! {
        // TODO(nkoturovic) No need to leave #input, only used as a blueprint
        #input
        // #[derive(Model, Debug)]
        // #ormlite(table = "people", insertable = InsertPerson)]
        #[derive(Debug)]
        struct #model_struct_name{
            #( #input_fields_iter, )*
        }
    })
}
