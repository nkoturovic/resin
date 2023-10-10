extern crate proc_macro;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemStruct};

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
    let create_struct_name = format_ident!("{}Create", input_struct_name);

    let create_struct = if resin.ops.contains('C') {
        quote! {
            #[derive(Debug, Serialize)]
            pub struct #create_struct_name{
                #( pub #input_fields_iter, )*
            }
        }
    } else {
        quote!()
    };

    TokenStream::from(quote! {
        #input // don't touch the original struct

        use serde::{Serialize,Deserialize};
        #create_struct
    })
}
