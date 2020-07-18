use crate::helpers::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{Data, DeriveInput};

pub fn visitable_impl(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let path = quote!(widgets::visitor);

    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(::widgets::widget::Widget)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data) => {
            let child_fields = find_tagged_fields(&data.fields, "visit_child");
            let iter_fields = find_tagged_fields(&data.fields, "visit_iter");

            let mut expanded: Vec<_> = child_fields
                .iter()
                .map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            visitor.visit_child(&mut self.#field, ctx, &pdata)?;
                        },
                    )
                })
                .chain(iter_fields.iter().map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            for child in &mut self.#field {
                                visitor.visit_child(child, ctx, &pdata)?;
                            }
                        },
                    )
                }))
                .collect();

            let mut expanded_rev: Vec<_> = child_fields
                .iter()
                .map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            visitor.visit_child_rev(&mut self.#field, ctx, &pdata)?;
                        },
                    )
                })
                .chain(iter_fields.iter().map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            for child in self.#field.iter_mut().rev() {
                                visitor.visit_child_rev(child, ctx, &pdata)?;
                            }
                        },
                    )
                }))
                .collect();

            expanded.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
            expanded_rev.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));

            let stmts = expanded.into_iter().map(|(_, s)| s);
            let stmts_rev = expanded_rev.into_iter().map(|(_, s)| s);

            Ok(quote! {
                impl #impl_generics #path::Visitable for #name #ty_generics #where_clause {
                    #[inline]
                    fn accept<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        let pdata = #path::ParentData::new(self);
                        visitor.visit(self, ctx)?;
                        #(#stmts)*
                        Ok(())
                    }

                    #[inline]
                    fn accept_rev<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        let pdata = #path::ParentData::new(self);
                        #(#stmts_rev)*
                        visitor.visit(self, ctx)
                    }
                }
            })
        }
        Data::Enum(data) => match_patterns_for_enum(&data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Visitable for #name #ty_generics #where_clause {
                    #[inline]
                    fn accept<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        match self {
                            #(#patterns => #path::Visitable::accept(a, visitor, ctx),)*
                        }
                    }

                    #[inline]
                    fn accept_rev<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        match self {
                            #(#patterns => #path::Visitable::accept_rev(a, visitor, ctx),)*
                        }
                    }
                }
            }
        }),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    expanded.unwrap_or_else(|err| err.to_error("Visitable").to_compile_error())
}
