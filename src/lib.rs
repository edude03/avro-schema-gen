use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use serde_json::json;
use syn::{
    parse_macro_input, token::Token, Data::Struct, DataStruct, DeriveInput, Fields, FieldsNamed,
    Type,
};

// #[derive(AvroSchema)]
// struct Foo {
//     bar: String,
//     baz: i32,
// }

trait AvroSchema {
    fn to_schema() -> serde_json::Value;
}

// enum AvroType {
//     String(String),
// }

// struct AvroField {
//     name: String,
//     r#type: String,
// }

// fn to_avro_type(t: Type) -> AvroType {
//     match t {
//         syn::Type::String { s, .. } => AvroType::String(s),
//         _ => panic!("Unsupported"),
//     }
// }

#[proc_macro_derive(AvroSchema)]
pub fn generate_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let outname = name.to_string();
    let data = input.data.clone();

    let fields = match data {
        Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let f = fields.clone();
            f.named
        }
        _ => panic!("expected a struct with named fields"),
    };

    let fs = fields.iter().map(|field| {
        let ident = field.ident.to_owned().unwrap();
        let ty = field.ty.clone().into_token_stream().to_string();
        let hy = ident.to_string();

        let out = quote!(json!({
          "name": #hy,
          "type": #ty
        }));

        println!("token sr {}", out.to_string());

        out
    });

    let fields = quote! { vec![#(#fs),*]};

    println!("{:?}", fields.to_string());
    let tokens = quote::quote! {
      impl AvroSchema for #name {
        fn to_schema(&self) -> serde_json::Value {
          json!({
            "name": #outname,
            "type": "record",
            "fields": #fields
          })
        }
      }
    };

    println!("Hi");

    TokenStream::from(tokens)
}
