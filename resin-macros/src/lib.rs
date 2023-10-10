extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parser, parse_macro_input, punctuated::Punctuated, Expr, ExprLit, ItemStruct, Lit,
    LitStr,
};

// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
pub fn resin_model(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_parsed =
        syn::punctuated::Punctuated::<syn::ExprAssign, syn::Token![,]>::parse_terminated
            .parse(args)
            .unwrap(); // Better to turn it into a `compile_error!()`

    let xs: Vec<(String, String)> = args_parsed
        .iter()
        .map(|arg| {
            if let Expr::Path(lhs) = arg.left.as_ref() {
                let key = lhs.path.segments.first().unwrap().ident.to_string();
                let value = match key.as_str() {
                    "ops" => {
                        if let Expr::Lit(rhs) = arg.right.as_ref() {
                            if let Lit::Str(rhs_str_lit) = rhs.lit.clone() {
                                rhs_str_lit
                                    .token()
                                    .to_string()
                                    .strip_prefix("\"")
                                    .unwrap()
                                    .strip_suffix("\"")
                                    .unwrap()
                                    .to_string()
                            } else {
                                panic!("Expected String lit, found {:#?}", rhs);
                            }
                        } else {
                            panic!("Invalid rhs expr {:#?}", arg.right);
                        }
                    }
                    _ => panic!("Invalid key {}", key),
                };
                (key, value)
            } else {
                panic!("Expected key=val, found {:#?}", arg);
            }
        })
        .collect();

    // println!("{:#?}", xs);
    let (_, ops_str) = xs.iter().find(|(k, _)| k == "ops").unwrap();

    // Parse the input as an item (expecting a struct definition).
    let input = parse_macro_input!(input as ItemStruct);
    let input_struct_name = &input.ident;
    let input_fields_iter = input.fields.iter();
    let create_struct_name = format_ident!("{}Create", input_struct_name);

    let create_struct = if ops_str.contains('C') {
        quote! {
            #[derive(Debug, Serialize)]
            pub struct #create_struct_name{
                #( pub #input_fields_iter, )*
            }
        }
    } else {
        quote!()
    };

    println!("{}", ops_str);

    TokenStream::from(quote! {
        #input // don't touch the original struct

        use serde::{Serialize,Deserialize};
        // Create Entity
        #create_struct
    })
}
