use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

mod bounds;
mod helpers;
mod object_id;
mod visitable;
mod widget;

#[proc_macro_derive(ObjectId, attributes(object_id, impl_generics))]
pub fn derive_object_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    object_id::object_id_impl(input).into()
}

#[proc_macro_derive(Bounds, attributes(bounds, position, size, impl_generics))]
pub fn derive_bounds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    bounds::bounds_impl(input).into()
}

#[proc_macro_derive(Visitable, attributes(visit_child, visit_iter, impl_generics))]
pub fn derive_visitable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    visitable::visitable_impl(input).into()
}

#[proc_macro_derive(Widget, attributes(impl_generics))]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    widget::widget_impl(input).into()
}
