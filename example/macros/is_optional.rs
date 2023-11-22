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

    let partial_struct_name = format_ident!("{}Partial", input_struct_name);
    let partial_struct_fields = input_fields_iter.map(|field| {
        if is_option(&field.ty) {
            quote! {
                #field
            }
        } else {
            let field_name = &field.ident;
            let field_type = &field.ty;
            quote! {
                #field_name: Option<#field_type>
            }
        }
    });

    let partial_struct = if resin.ops.contains('C') {
        quote! {
            #[derive(Debug, Serialize)]
            pub struct #partial_struct_name{
                #( pub #partial_struct_fields, )*
            }
        }
    } else {
        quote!()
    };

    TokenStream::from(quote! {
        #input // don't touch the original struct

        use serde::{Serialize,Deserialize};
        #partial_struct
    })
}

fn check_for_option<'t>(path: &'t Path) -> Option<&'t syn::PathSegment> {
    let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push(':');
        acc
    });
    vec!["Option:", "std:option:Option:", "core:option:Option:"]
        .into_iter()
        .find(|s| idents_of_path == *s)
        .and_then(|_| path.segments.last())
}

fn is_option(typ: &Type) -> bool {
    let opt = match typ {
        Type::Path(typepath) if typepath.qself.is_none() => Some(typepath.path.clone()),
        _ => None,
    };

    if let Some(o) = opt {
        check_for_option(&o).is_some()
    } else {
        false
    }
}
