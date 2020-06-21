use quote::quote;
use syn::parse_macro_input;
use syn::{Data, DeriveInput};

mod helpers;
use helpers::*;

const WIDGET_ID: &str = "WidgetId";

#[proc_macro_derive(ObjectId)]
pub fn object_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let path = quote!(::widgets::widget);

    let expanded = match &input.data {
        Data::Struct(data) => find_field_struct(data, &name, WIDGET_ID).map(|field| {
            quote! {
                impl #impl_generics #path::ObjectId for #name #ty_generics #where_clause {
                    fn get_id(&self) -> #path::WidgetId {
                        self.#field
                    }
                }
            }
        }),
        Data::Enum(data) => find_field_enum(data, &name, WIDGET_ID).map(|patterns| {
            quote! {
                impl #impl_generics #path::ObjectId for #name #ty_generics #where_clause {
                    fn get_id(&self) -> #path::WidgetId {
                        match self {
                            #(#patterns => a.get_id(),)*
                        }
                    }
                }
            }
        }),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "ObjectId for union")),
    };

    expanded
        .unwrap_or_else(|err| err.to_error(WIDGET_ID).to_compile_error())
        .into()
}
