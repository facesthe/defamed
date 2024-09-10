//! Various methods for manipulating a particular block of code.
//!

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::ToTokens;

use crate::{macro_gen, params};

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
pub fn item_fn(input: syn::ItemFn) -> ProcOutput {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let params = match params::FunctionParams::from_punctuated(sig.inputs.clone()) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    let permuted = params.permute_params();
    let stripped_attrs = params.to_punctuated();
    let mut new_sig = sig.clone();
    new_sig.inputs = stripped_attrs;

    let generated = macro_gen::generate_func_macro(new_sig.ident.clone(), permuted);

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
