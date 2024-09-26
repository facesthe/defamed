use quote::ToTokens;
use syn::punctuated::Punctuated;

use super::ParamAttr;

/// Parsed struct fields
#[derive(Clone)]
pub struct StructFields {
    ident: syn::Ident,
    fields: Vec<StructField>,
}

/// A parsed struct field.
/// For tuple structs, each field does not have an identitifer.
#[derive(Clone)]
pub struct StructField {
    vis: syn::Visibility,
    ident: Option<syn::Ident>,
    ty: syn::Type,
    default_value: ParamAttr,
}

/// Permutation of struct fields
#[derive(Clone)]
pub enum PermutedField {
    // this should be rarely used
    Positional(StructField),
    Named(StructField),
    Default(StructField),
}

impl PartialEq for StructField {
    fn eq(&self, other: &Self) -> bool {
        self.ident.to_token_stream().to_string() == other.ident.to_token_stream().to_string()
    }
}

impl PartialEq for PermutedField {
    fn eq(&self, other: &Self) -> bool {
        let inner = match self {
            Self::Positional(_i) => _i,
            Self::Named(_i) => _i,
            Self::Default(_i) => _i,
        };

        let othr = match other {
            Self::Positional(_i) => _i,
            Self::Named(_i) => _i,
            Self::Default(_i) => _i,
        };

        inner == othr
    }
}

impl StructFields {
    pub fn from_named(ident: syn::Ident, fields: Punctuated<syn::Field, syn::Token![,]>) -> Self {
        let fields = fields
            .into_iter()
            .map(StructField::from_field_type)
            .collect();
        Self { ident, fields }
    }
}

impl StructField {
    /// Parse a struct field into `Self`.
    pub fn from_field_type(field: syn::Field) -> Self {
        Self {
            vis: field.vis,
            ident: field.ident,
            ty: field.ty,
            default_value: ParamAttr::None,
        }
    }
}
