use quote::quote;
use syn::{parse_macro_input, parse_quote};
use syn::{Data, DeriveInput};

mod helpers;
use helpers::*;

#[proc_macro_derive(ObjectId, attributes(object_id, impl_generics))]
pub fn derive_object_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
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
                fn get_id(&self) -> #path::WidgetId {
                    #body
                }
            }
        }
    })
    .unwrap_or_else(|err| err.to_error("ObjectId").to_compile_error())
    .into()
}

#[proc_macro_derive(Bounds, attributes(bounds, position, size, impl_generics))]
pub fn derive_bounds(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let path = quote!(widgets::geometry);

    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(#path::Bounds)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data) => match find_field_in_struct(data, &name, "Rect", "bounds") {
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
                let pos_res = find_field_in_struct(data, &name, "Position", "position");
                let size_res = find_field_in_struct(data, &name, "Size", "size");

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
        Data::Enum(data) => match_patterns_for_enum(data, &name).map(|patterns| {
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

#[proc_macro_derive(Visitable, attributes(visit_child, visit_iter, impl_generics))]
pub fn derive_visitable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let path = quote!(widgets::visitor);

    if let Err(err) = parse_impl_generics(
        &input.attrs,
        &mut input.generics,
        parse_quote!(::widgets::widget::Widget),
    ) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data) => {
            let child_fields = find_tagged_fields(&data.fields, "visit_child");
            let iter_fields = find_tagged_fields(&data.fields, "visit_iter");

            let mut expanded: Vec<_> = child_fields
                .iter()
                .map(|(i, field)| (*i, quote! { visitor.visit_child(&mut self.#field, ctx)?; }))
                .chain(iter_fields.iter().map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            for child in &mut self.#field {
                                visitor.visit_child(child, ctx)?;
                            }
                        },
                    )
                }))
                .collect();

            let mut expanded_rev: Vec<_> = child_fields
                .iter()
                .map(|(i, field)| (*i, quote! { visitor.visit_child(&mut self.#field, ctx)?; }))
                .chain(iter_fields.iter().map(|(i, field)| {
                    (
                        *i,
                        quote! {
                            for child in self.#field.iter_mut().rev() {
                                visitor.visit_child(child, ctx)?;
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
                    fn accept<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        visitor.visit(self, ctx)?;
                        #(#stmts)*
                        Ok(())
                    }

                    fn accept_rev<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        #(#stmts_rev)*
                        visitor.visit(self, ctx)
                    }
                }
            })
        }
        Data::Enum(data) => match_patterns_for_enum(&data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Visitable for #name #ty_generics #where_clause {
                    fn accept<V: #path::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                        match self {
                            #(#patterns => #path::Visitable::accept(a, visitor, ctx),)*
                        }
                    }

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

    expanded
        .unwrap_or_else(|err| err.to_error("Visitable").to_compile_error())
        .into()
}

#[proc_macro_derive(Widget, attributes(impl_generics))]
pub fn derive_widget(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let path = quote!(widgets::widget);
    let pevent = quote!(widgets::event);
    let crate_ = quote!(widgets);

    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(#path::Widget)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Enum(data) => match_patterns_for_enum(&data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Widget for #name #ty_generics #where_clause {
                    fn update_layout(&mut self, parent_rect: #crate_::geometry::Rect) {
                        match self {
                            #(#patterns => #path::Widget::update_layout(a, parent_rect),)*
                        }
                    }

                    fn draw(&self, dc: #crate_::draw::DrawContext) {
                        match self {
                            #(#patterns => #path::Widget::draw(a, dc),)*
                        }
                    }

                    fn handle_event(&mut self, event: &#pevent::Event, ctx: #pevent::EventContext) -> #pevent::EventResult {
                        match self {
                            #(#patterns => #path::Widget::handle_event(a, event, ctx),)*
                        }
                    }

                    fn event_consumed(&mut self, wid: #path::WidgetId, event: &#pevent::Event, ctx: #pevent::EventContext) {
                        match self {
                            #(#patterns => #path::Widget::event_consumed(a, wid, event, ctx),)*
                        }
                    }
                }
            }
        }),
        Data::Struct(data) => Err(FieldFindError::Unsupported(data.struct_token.span, "struct")),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    expanded
        .unwrap_or_else(|err| err.to_error("Widget").to_compile_error())
        .into()
}
