use crate::helpers::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{Data, DeriveInput};

pub fn bounds_impl(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let path = quote!(widgets::geometry);

    let mut generics_mut = input.generics.clone();
    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(#path::Bounds)) {
        return err.to_compile_error().into();
    }
    if let Err(err) = parse_impl_generics(&input.attrs, &mut generics_mut, parse_quote!(#path::BoundsMut)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_mut, ty_generics_mut, where_clause_mut) = generics_mut.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data) => match find_field_in_struct(data, &name, "Rect", "bounds") {
            Ok(field) => Ok(quote! {
                impl #impl_generics #path::Bounds for #name #ty_generics #where_clause {
                    #[inline]
                    fn get_position(&self) -> #path::Position {
                        #path::Bounds::get_position(&self.#field)
                    }

                    #[inline]
                    fn get_size(&self) -> #path::Size {
                        #path::Bounds::get_size(&self.#field)
                    }

                    #[inline]
                    fn get_bounds(&self) -> #path::Rect {
                        #path::Bounds::get_bounds(&self.#field)
                    }
                }

                impl #impl_generics_mut #path::BoundsMut for #name #ty_generics_mut #where_clause_mut {
                    #[inline]
                    fn set_position(&mut self, position: #path::Position) {
                        #path::BoundsMut::set_position(&mut self.#field, position)
                    }

                    #[inline]
                    fn set_size(&mut self, size: #path::Size) {
                        #path::BoundsMut::set_size(&mut self.#field, size)
                    }

                    #[inline]
                    fn set_bounds(&mut self, bounds: #path::Rect) {
                        #path::BoundsMut::set_bounds(&mut self.#field, bounds)
                    }
                }
            }),
            Err(FieldFindError::NotFound(rerr, rname)) => {
                let pos_res = find_field_in_struct(data, &name, "Position", "position");
                let size_res = find_field_in_struct(data, &name, "Size", "size");

                match (pos_res, size_res) {
                    (Ok(pos), Ok(size)) => Ok(quote! {
                        impl #impl_generics #path::Bounds for #name #ty_generics #where_clause {
                            #[inline]
                            fn get_position(&self) -> #path::Position {
                                self.#pos
                            }

                            #[inline]
                            fn get_size(&self) -> #path::Size {
                                self.#size
                            }
                        }

                        impl #impl_generics_mut #path::BoundsMut for #name #ty_generics_mut #where_clause_mut {
                            #[inline]
                            fn set_position(&mut self, position: #path::Position) {
                                self.#pos = position;
                            }

                            #[inline]
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
        Data::Enum(data) => match_patterns_for_enum(data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Bounds for #name #ty_generics #where_clause {
                    #[inline]
                    fn get_position(&self) -> #path::Position {
                        match self {
                            #(#patterns => #path::Bounds::get_position(a),)*
                        }
                    }

                    #[inline]
                    fn get_size(&self) -> #path::Size {
                        match self {
                            #(#patterns => #path::Bounds::get_size(a),)*
                        }
                    }

                    #[inline]
                    fn get_bounds(&self) -> #path::Rect {
                        match self {
                            #(#patterns => #path::Bounds::get_bounds(a),)*
                        }
                    }
                }

                impl #impl_generics_mut #path::BoundsMut for #name #ty_generics_mut #where_clause_mut {
                    #[inline]
                    fn set_position(&mut self, position: #path::Position) {
                        match self {
                            #(#patterns => #path::BoundsMut::set_position(a, position),)*
                        }
                    }

                    #[inline]
                    fn set_size(&mut self, size: #path::Size) {
                        match self {
                            #(#patterns => #path::BoundsMut::set_size(a, size),)*
                        }
                    }

                    #[inline]
                    fn set_bounds(&mut self, bounds: #path::Rect) {
                        match self {
                            #(#patterns => #path::BoundsMut::set_bounds(a, bounds),)*
                        }
                    }
                }
            }
        }),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    expanded.unwrap_or_else(|err| err.to_error("Bounds").to_compile_error())
}
