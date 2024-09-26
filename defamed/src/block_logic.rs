//! Various methods for manipulating a particular block of code.

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::{quote, ToTokens};

use crate::{
    macro_gen,
    permute::{fields::StructFields, params},
};

/// Output of a processing function
pub struct ProcOutput {
    /// Modified code to be substituted in-place
    pub modified: pm2::TokenStream,
    /// Generated code to be appended to the end of macro invocation
    pub generated: pm2::TokenStream,
}

impl From<pm::TokenStream> for ProcOutput {
    fn from(value: pm::TokenStream) -> Self {
        Self {
            modified: value.into(),
            generated: Default::default(),
        }
    }
}

impl From<pm2::TokenStream> for ProcOutput {
    fn from(value: pm2::TokenStream) -> Self {
        Self {
            modified: value,
            generated: Default::default(),
        }
    }
}

impl From<ProcOutput> for pm::TokenStream {
    fn from(value: ProcOutput) -> Self {
        let mut modified = value.modified;
        modified.extend(value.generated);

        modified.into()
    }
}

/// Process a standalone function.
/// The crate path of the funciton is passed as an optional parameter.
pub fn item_fn(input: syn::ItemFn, fn_path: Option<syn::Path>) -> ProcOutput {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    // check visibility vs provided path
    match (&vis, fn_path.as_ref()) {
        (syn::Visibility::Restricted(syn::VisRestricted { path, .. }), None) => {
            if !path.is_ident("self") {
                return syn::Error::new(
                    sig.ident.span(),
                    "Attribute requires a path to the function for public functions",
                )
                .to_compile_error()
                .into();
            }
        }
        (syn::Visibility::Public(_), None) => {
            return syn::Error::new(
                sig.ident.span(),
                "Attribute requires a path to the function for public functions",
            )
            .to_compile_error()
            .into();
        }
        _ => (),
    }

    let params = match params::FunctionParams::from_punctuated(sig.inputs.clone()) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    if let Some(invalid) = params.first_invalid_param() {
        return syn::Error::new(
            invalid.inner_span(),
            "Default parameters must be placed after all positional parameters",
        )
        .to_compile_error()
        .into();
    }

    let permuted = params.permute_params();
    let new_args = params.to_punctuated();
    let mut new_sig = sig.clone();
    new_sig.inputs = new_args;

    // let doc_attrs = attrs
    //     .iter()
    //     .cloned()
    //     .filter(|a| a.path().is_ident("doc"))
    //     .collect::<Vec<_>>();

    let generated = macro_gen::generate_func_macro(
        vis.clone(),
        // doc_attrs,
        // package_name,
        fn_path,
        new_sig.ident.clone(),
        permuted,
    );

    let mod_fn = syn::ItemFn {
        attrs,
        vis,
        sig: new_sig,
        block,
    }
    .to_token_stream();

    ProcOutput {
        modified: mod_fn,
        generated,
    }
}

/// Process a struct definition
pub fn item_struct(input: syn::ItemStruct, s_path: Option<syn::Path>) -> ProcOutput {
    match input.fields {
        syn::Fields::Named(named_fields) => item_struct_struct(
            s_path,
            input.attrs,
            input.vis,
            input.ident,
            input.generics,
            named_fields,
        ),
        syn::Fields::Unnamed(unnamed_fields) => item_struct_tuple(
            s_path,
            input.attrs,
            input.vis,
            input.ident,
            input.generics,
            unnamed_fields,
        ),
        syn::Fields::Unit => {
            return {
                let warning = proc_macro_warning::FormattedWarning::new_deprecated(
                    "IrrelevantMacro",
                    "Remove this attribute macro. Unit structs do not contain any fields and cannot have default parameters.",
                    input.ident.span(),
                );

                quote! {
                    #warning
                }
                .into()
            }
        }
    }
}

/// Process a normal struct
fn item_struct_struct(
    s_path: Option<syn::Path>,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: syn::Ident,
    generics: syn::Generics,
    fields: syn::FieldsNamed,
) -> ProcOutput {
    // asd

    match (&vis, s_path.as_ref()) {
        (syn::Visibility::Restricted(syn::VisRestricted { path, .. }), None) => {
            if !path.is_ident("self") {
                return syn::Error::new(
                    ident.span(),
                    "Attribute requires a path to the struct for public structs",
                )
                .to_compile_error()
                .into();
            }
        }
        (syn::Visibility::Public(_), None) => {
            return syn::Error::new(
                ident.span(),
                "Attribute requires a path to the struct for public structs",
            )
            .to_compile_error()
            .into();
        }
        _ => (),
    }

    let n_fields = StructFields::from_named(ident.clone(), fields.named.clone());

    ProcOutput {
        modified: syn::ItemStruct {
            attrs,
            vis,
            struct_token: Default::default(),
            ident,
            generics,
            fields: syn::Fields::Named(fields),
            semi_token: None,
        }
        .to_token_stream(),
        generated: quote! {},
    }
}

/// Process a tuple struct
fn item_struct_tuple(
    s_path: Option<syn::Path>,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: syn::Ident,
    generics: syn::Generics,
    fields: syn::FieldsUnnamed,
) -> ProcOutput {
    ProcOutput {
        modified: syn::ItemStruct {
            attrs,
            vis,
            struct_token: Default::default(),
            ident,
            generics,
            fields: syn::Fields::Unnamed(fields),
            semi_token: None,
        }
        .to_token_stream(),
        generated: quote! {},
    }
}

#[allow(unused)]
fn impl_item_fn(input: syn::ImplItemFn) {
    // this is the only thing that is different from item_fn
    let def_ness = input.defaultness;

    let item_as_fn = syn::ItemFn {
        attrs: input.attrs,
        vis: input.vis,
        sig: input.sig,
        block: Box::new(input.block),
    };
}

/// Processes all functions inside an `impl` block
#[allow(unused)]
pub fn item_impl(input: syn::ItemImpl) -> ProcOutput {
    let inter = input.items.into_iter().map(|item| match item {
        syn::ImplItem::Const(_) => todo!(),
        syn::ImplItem::Fn(f) => {}
        syn::ImplItem::Type(_) => todo!(),
        syn::ImplItem::Macro(_) => todo!(),
        syn::ImplItem::Verbatim(_) => todo!(),
        _ => todo!(),
    });

    todo!()
}

#[allow(dead_code)]
pub fn item_mod(_input: syn::ItemMod) -> ProcOutput {
    todo!()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn test_match_impl_block() {
        let tokens = quote! {
            impl SomeStruct {
                pub fn new() -> Self {
                    SomeStruct {}
                }
            }
        };

        let _: syn::ItemImpl = syn::parse2(tokens).unwrap();
    }

    #[test]
    fn test_match_mod_block() {
        let tokens = quote! {
            mod some_module {
                struct X{}
            }
        };

        let _: syn::ItemMod = syn::parse2(tokens).unwrap();
    }
}
