use crate::helpers::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{Data, DeriveInput};

pub fn widget_impl(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let crate_ = quote!(widgets);
    let path = quote!(widgets::widget);
    let pevent = quote!(widgets::event);

    if let Err(err) = parse_impl_generics(&input.attrs, &mut input.generics, parse_quote!(#path::Widget)) {
        return err.to_compile_error().into();
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        Data::Enum(data) => match_patterns_for_enum(&data, &name).map(|patterns| {
            quote! {
                impl #impl_generics #path::Widget for #name #ty_generics #where_clause {
                    #[inline]
                    fn update_layout(&mut self, parent_rect: #crate_::geometry::Rect) {
                        match self {
                            #(#patterns => #path::Widget::update_layout(a, parent_rect),)*
                        }
                    }

                    #[inline]
                    fn draw<V: #crate_::draw::Vertex>(&self, dc: #crate_::draw::DrawContext<V>) {
                        match self {
                            #(#patterns => #path::Widget::draw(a, dc),)*
                        }
                    }

                    #[inline]
                    fn handle_event(&mut self, event: &#pevent::Event, ctx: #pevent::EventContext) -> #pevent::EventResult {
                        match self {
                            #(#patterns => #path::Widget::handle_event(a, event, ctx),)*
                        }
                    }

                    #[inline]
                    fn event_consumed(&mut self, event: &#pevent::Event, ctx: &#pevent::EventContext) {
                        match self {
                            #(#patterns => #path::Widget::event_consumed(a, event, ctx),)*
                        }
                    }

                    #[inline]
                    fn viewport_origin(&self) -> #crate_::geometry::Position {
                        match self {
                            #(#patterns => #path::Widget::viewport_origin(a),)*
                        }
                    }
                }
            }
        }),
        Data::Struct(data) => Err(FieldFindError::Unsupported(data.struct_token.span, "struct")),
        Data::Union(data) => Err(FieldFindError::Unsupported(data.union_token.span, "union")),
    };

    expanded.unwrap_or_else(|err| err.to_error("Widget").to_compile_error())
}