extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

struct TSConfig {
    export_path: String,
    file_extension: String,
}

struct TSTypes {
    struct_name: String,
    fields: HashMap<String, String>,
}

static TS_TYPES: Vec<TSTypes> = vec![];

#[proc_macro_attribute]
pub fn ts_type(args: TokenStream, item: TokenStream) -> TokenStream {
    // -> args
    // -> TokenStream
    // -> Iter<TokenTree>
    // -> TokenTree
    // -> Enum<>
    args.into_iter().for_each(|tree| println!("{:?}", tree));

    let my_cloned_item = item.clone();
    let input = syn::parse_macro_input!(my_cloned_item as syn::DeriveInput);
    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                let this_thing = quote! { stringify(!(#identifier)) };
                println!("{}", this_thing);
            }
        }
        Data::Enum(_) => todo!(),
        _ => panic!("Unions not supported type"),
    }
    item
}
