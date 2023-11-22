extern crate proc_macro;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct, Path, Type};

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
struct Resin {
    ops: String,
}

// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
pub fn resin_model(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let resin = match Resin::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    // Parse the input as an item (expecting a struct definition).
    let input = parse_macro_input!(input as ItemStruct);
    let input_struct_name = &input.ident;
    let input_fields_iter = input.fields.iter();

    let perms_struct_name = format_ident!("{}Perms", input_struct_name);
    let perms_struct_fields = input_fields_iter.map(|field| {
        let field_name = &field.ident;
        // let field_type = &field.ty;
        quote! {
            #field_name: u8
        }
    });

    TokenStream::from(quote! {
        #input // don't touch the original struct

        #[derive(Debug)]
        struct #perms_struct_name {
            #(#perms_struct_fields),*
        }
    })
}
