use quote::quote;
use syn::parse_macro_input;
use syn::{Data, DeriveInput};

mod helpers;
use helpers::*;

#[proc_macro_derive(ObjectId, attributes(object_id))]
pub fn derive_object_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let path = quote!(::widgets::widget);

    let body = match &input.data {
        Data::Struct(data) => find_field_struct(data, &name, "WidgetId", "object_id").map(|field| {
            quote! { #path::ObjectId::get_id(&self.#field) }
        }),
        Data::Enum(data) => find_field_enum(data, &name).map(|patterns| {
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
                fn get_id(&self) -> #path::WidgetId {
                    #body
                }
            }
        }
    })
    .unwrap_or_else(|err| err.to_error("ObjectId").to_compile_error())
    .into()
}

#[proc_macro_derive(Bounds, attributes(bounds, position, size))]
pub fn derive_bounds(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let path = quote!(::widgets::geometry);

    let expanded = match &input.data {
        Data::Struct(data) => match find_field_struct(data, &name, "Rect", "bounds") {
            Ok(field) => Ok(quote! {
                impl #impl_generics #path::Bounds for #name #ty_generics #where_clause {
                    fn get_position(&self) -> #path::Position {
                        #path::Bounds::get_position(&self.#field)
                    }

                    fn get_size(&self) -> #path::Size {
                        #path::Bounds::get_size(&self.#field)
                    }

                    fn set_position(&mut self, position: #path::Position) {
                        #path::Bounds::set_position(&mut self.#field, position)
                    }

                    fn set_size(&mut self, size: #path::Size) {
                        #path::Bounds::set_size(&mut self.#field, size)
                    }

                    fn get_bounds(&self) -> #path::Rect {
                        #path::Bounds::get_bounds(&self.#field)
                    }
                }
            }),
            Err(FieldFindError::NotFound(rerr, rname)) => {
                let pos_res = find_field_struct(data, &name, "Position", "position");
                let size_res = find_field_struct(data, &name, "Size", "size");

                match (pos_res, size_res) {
                    (Ok(pos), Ok(size)) => Ok(quote! {
                        impl #impl_generics #path::Bounds for #name #ty_generics #where_clause {
                            fn get_position(&self) -> #path::Position {
                                self.#pos
                            }

                            fn get_size(&self) -> #path::Size {
                                self.#size
                            }

                            fn set_position(&mut self, position: #path::Position) {
                                self.#pos = position;
                            }

                            fn set_size(&mut self, size: #path::Size) {
                                self.#size = size;
                            }
                        }
                    }),
                    (Ok(_), Err(err)) | (Err(err), Ok(_)) => Err(err),
                    (Err(_), Err(_)) => Err(FieldFindError::NotFound(rerr, rname)),
                }
            }
            other => other,
        },
        Data::Enum(data) => find_field_enum(data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Bounds for #name #ty_generics #where_clause {
                    fn get_position(&self) -> #path::Position {
                        match self {
                            #(#patterns => #path::Bounds::get_position(a),)*
                        }
                    }

                    fn get_size(&self) -> #path::Size {
                        match self {
                            #(#patterns => #path::Bounds::get_size(a),)*
                        }
                    }

                    fn set_position(&mut self, position: #path::Position) {
                        match self {
                            #(#patterns => #path::Bounds::set_position(a, position),)*
                        }
                    }

                    fn set_size(&mut self, size: #path::Size) {
                        match self {
                            #(#patterns => #path::Bounds::set_size(a, size),)*
                        }
                    }

                    fn get_bounds(&self) -> #path::Rect {
                        match self {
                            #(#patterns => #path::Bounds::get_bounds(a),)*
                        }
                    }
                }
            }
        }),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    expanded
        .unwrap_or_else(|err| err.to_error("Bounds").to_compile_error())
        .into()
}
