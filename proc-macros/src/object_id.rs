use crate::helpers::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{Data, DeriveInput};

pub fn object_id_impl(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let path = quote!(widgets::widget);

    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(#path::ObjectId)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) => find_field_in_struct(data, &name, "WidgetId", "object_id").map(|field| {
            quote! { #path::ObjectId::get_id(&self.#field) }
        }),
        Data::Enum(data) => match_patterns_for_enum(data, &name).map(|patterns| {
            quote! {
                match self {
                    #(#patterns => #path::ObjectId::get_id(a),)*
                }
            }
        }),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    body.map(|body| {
        quote! {
            impl #impl_generics #path::ObjectId for #name #ty_generics #where_clause {
                #[inline]
                fn get_id(&self) -> #path::WidgetId {
                    #body
                }
            }
        }
    })
    .unwrap_or_else(|err| err.to_error("ObjectId").to_compile_error())
}
