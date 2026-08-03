use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Entity, attributes(flyer_orm))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Ensure we are working with a struct with named fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("#[derive(Entity)] only supports structs with named fields"),
        },
        _ => panic!("#[derive(Entity)] can only be used on structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        // 1. Implement sqlx::FromRow under the hood for any database row type R
        impl<'r, R> ::sqlx::FromRow<'r, R> for #name #ty_generics
        where
            R: ::sqlx::Row,
            &'r str: ::sqlx::ColumnIndex<R>,
            #( #field_types: ::sqlx::Type<<R as ::sqlx::Row>::Database> + ::sqlx::Decode<'r, <R as ::sqlx::Row>::Database>, )*
            #where_clause
        {
            fn from_row(row: &'r R) -> ::std::result::Result<Self, ::sqlx::Error> {
                ::std::result::Result::Ok(Self {
                    #(
                        #field_names: row.try_get(stringify!(#field_names))?,
                    )*
                })
            }
        }

        // 2. Implement your clean marker trait
        impl #impl_generics Entity for #name #ty_generics #where_clause {}
    };

    return TokenStream::from(expanded);
}