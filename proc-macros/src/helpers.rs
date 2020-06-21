use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Type};

pub enum FieldFindError {
    Duplicate(Span),
    NotFound(Span),
    Empty(Span),
    Unsupported(Span, &'static str),
}

impl FieldFindError {
    pub fn to_error(self, name: &str) -> Error {
        match self {
            FieldFindError::Duplicate(span) => Error::new(span, format!("duplicate {} field", name)),
            FieldFindError::NotFound(span) => Error::new(span, format!("{} field not found", name)),
            FieldFindError::Empty(span) => Error::new(span, format!("can't derive empty type (looking for {})", name)),
            FieldFindError::Unsupported(span, desc) => Error::new(span, format!("can't derive {}", desc)),
        }
    }
}

pub fn find_named_field(fields: &FieldsNamed, name: &str) -> Result<TokenStream, FieldFindError> {
    let mut field_found = None;

    for field in &fields.named {
        if let Type::Path(path) = &field.ty {
            if let Some(ty_name) = path.path.segments.iter().last() {
                if ty_name.ident == name {
                    if field_found.is_some() {
                        return Err(FieldFindError::Duplicate(field.ident.as_ref().unwrap().span()));
                    }
                    field_found = Some(field.ident.to_token_stream());
                }
            }
        }
    }

    field_found.ok_or(FieldFindError::NotFound(fields.brace_token.span))
}

pub fn find_unnamed_field(fields: &FieldsUnnamed, name: &str) -> Result<TokenStream, FieldFindError> {
    let mut field_found = None;

    for (i, field) in fields.unnamed.iter().enumerate() {
        if let Type::Path(path) = &field.ty {
            if let Some(ty_name) = path.path.segments.iter().last() {
                if ty_name.ident == name {
                    if field_found.is_some() {
                        return Err(FieldFindError::Duplicate(ty_name.ident.span()));
                    }
                    field_found = Some(Index::from(i).to_token_stream());
                }
            }
        }
    }

    field_found.ok_or(FieldFindError::NotFound(fields.paren_token.span))
}

pub fn find_field_struct(data: &DataStruct, s_name: &Ident, ty_name: &str) -> Result<TokenStream, FieldFindError> {
    match &data.fields {
        Fields::Named(fields) => find_named_field(fields, ty_name),
        Fields::Unnamed(fields) => find_unnamed_field(fields, ty_name),
        Fields::Unit => Err(FieldFindError::Empty(s_name.span())),
    }
}

pub fn find_field_enum(data: &DataEnum, e_name: &Ident, ty_name: &str) -> Result<Vec<TokenStream>, FieldFindError> {
    data.variants
        .iter()
        .map(|variant| {
            match &variant.fields {
                Fields::Named(fields) => match find_named_field(fields, ty_name) {
                    Err(FieldFindError::NotFound(_)) => {
                        let first = fields.named.first().and_then(|f| f.ident.as_ref());
                        Ok(first.to_token_stream())
                    }
                    other => other,
                },
                Fields::Unnamed(fields) => match find_unnamed_field(fields, ty_name) {
                    Err(FieldFindError::NotFound(_)) => Ok(Index::from(0).to_token_stream()),
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
