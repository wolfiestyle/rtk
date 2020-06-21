use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Type};

type Str = &'static str;

pub enum FieldFindError {
    Duplicate(Span, Str),
    NotFound(Span, Str),
    Empty(Span),
    Unsupported(Span, Str),
}

impl FieldFindError {
    pub fn to_error(self, t_name: &str) -> Error {
        match self {
            FieldFindError::Duplicate(span, name) => Error::new(
                span,
                format!("found multiple `{}` typed fields while deriving `{}`", name, t_name),
            ),
            FieldFindError::NotFound(span, name) => Error::new(
                span,
                format!("field with `{}` type not found while deriving `{}`", name, t_name),
            ),
            FieldFindError::Empty(span) => Error::new(span, format!("can't derive `{}` on empty type", t_name)),
            FieldFindError::Unsupported(span, name) => {
                Error::new(span, format!("can't derive `{}` for `{}`", t_name, name))
            }
        }
    }
}

pub fn find_named_field(fields: &FieldsNamed, name: Str) -> Result<TokenStream, FieldFindError> {
    if fields.named.is_empty() {
        return Err(FieldFindError::Empty(fields.brace_token.span));
    }

    let mut field_found = None;
    for field in &fields.named {
        if let Type::Path(path) = &field.ty {
            if let Some(ty_name) = path.path.segments.iter().last() {
                if ty_name.ident == name {
                    if field_found.is_some() {
                        return Err(FieldFindError::Duplicate(field.ident.as_ref().unwrap().span(), name));
                    }
                    field_found = Some(field.ident.to_token_stream());
                }
            }
        }
    }

    field_found.ok_or(FieldFindError::NotFound(fields.brace_token.span, name))
}

pub fn find_unnamed_field(fields: &FieldsUnnamed, name: Str) -> Result<TokenStream, FieldFindError> {
    if fields.unnamed.is_empty() {
        return Err(FieldFindError::Empty(fields.paren_token.span));
    }

    let mut field_found = None;
    for (i, field) in fields.unnamed.iter().enumerate() {
        if let Type::Path(path) = &field.ty {
            if let Some(ty_name) = path.path.segments.iter().last() {
                if ty_name.ident == name {
                    if field_found.is_some() {
                        return Err(FieldFindError::Duplicate(ty_name.ident.span(), name));
                    }
                    field_found = Some(Index::from(i).to_token_stream());
                }
            }
        }
    }

    field_found.ok_or(FieldFindError::NotFound(fields.paren_token.span, name))
}

pub fn find_field_struct(data: &DataStruct, s_name: &Ident, ty_name: Str) -> Result<TokenStream, FieldFindError> {
    match &data.fields {
        Fields::Named(fields) => find_named_field(fields, ty_name),
        Fields::Unnamed(fields) => find_unnamed_field(fields, ty_name),
        Fields::Unit => Err(FieldFindError::Empty(s_name.span())),
    }
}

pub fn find_field_enum(data: &DataEnum, e_name: &Ident, ty_name: Str) -> Result<Vec<TokenStream>, FieldFindError> {
    data.variants
        .iter()
        .map(|variant| {
            match &variant.fields {
                Fields::Named(fields) => match find_named_field(fields, ty_name) {
                    Err(FieldFindError::NotFound(_, _)) => {
                        let first = fields.named.first().and_then(|f| f.ident.as_ref());
                        Ok(first.to_token_stream())
                    }
                    other => other,
                },
                Fields::Unnamed(fields) => match find_unnamed_field(fields, ty_name) {
                    Err(FieldFindError::NotFound(_, _)) => Ok(Index::from(0).to_token_stream()),
                    other => other,
                },
                Fields::Unit => Err(FieldFindError::Empty(variant.ident.span())),
            }
            .map(|field| {
                let vname = &variant.ident;
                quote! { #e_name::#vname { #field: a, .. } }
            })
        })
        .collect()
}
