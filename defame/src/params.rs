//! Function param stuff

use core::panic;
use std::clone;

use quote::ToTokens;
use syn::{punctuated, spanned::Spanned};

#[derive(Clone)]
pub struct FunctionParams {
    receiver: FnReceiver,
    params: Vec<FunctionParam>,
}

/// Default function parameter
#[derive(Clone)]
pub struct FunctionParam {
    /// Param name
    pat: syn::Pat,
    ty: syn::Type,
    /// A const that can be used as a default value
    default_value: ParamAttr,
}

/// Function parameter receiver
#[derive(Clone)]
pub enum FnReceiver {
    None,
    /// Self
    Slf {
        ty: syn::Type,
        token: syn::token::SelfValue,
        mutable: bool,
        reference: bool,
        lifetime: Option<syn::Lifetime>,
        colon_token: Option<syn::token::Colon>,
    },
}

#[derive(Clone)]
pub enum ParamAttr {
    /// No helper attribute
    None,
    // Use default trait for initialization
    Default,
    // Use const expr for initialization
    Value(syn::Expr),
}

impl FunctionParams {
    pub fn from_punctuated(
        punctuated: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    ) -> Result<Self, syn::Error> {
        let mut s = Self {
            receiver: FnReceiver::None,
            params: Vec::new(),
        };
        let mut has_receiver = false;

        for punct in punctuated {
            match punct {
                syn::FnArg::Receiver(recv) => {
                    if !has_receiver {
                        has_receiver = true;
                    } else {
                        panic!("Function cannot accept multiple receivers");
                    }

                    let receiver = match (&recv.reference, &recv.mutability) {
                        (None, None) => FnReceiver::Slf {
                            ty: *recv.ty.clone(),
                            token: recv.self_token,
                            mutable: false,
                            reference: false,
                            lifetime: recv.lifetime().cloned(),
                            colon_token: recv.colon_token,
                        },
                        (None, Some(_)) => FnReceiver::Slf {
                            ty: *recv.ty.clone(),
                            token: recv.self_token,
                            mutable: true,
                            reference: false,
                            lifetime: recv.lifetime().cloned(),
                            colon_token: recv.colon_token,
                        },
                        (Some(_), None) => FnReceiver::Slf {
                            ty: *recv.ty.clone(),
                            token: recv.self_token,
                            mutable: false,
                            reference: true,
                            lifetime: recv.lifetime().cloned(),
                            colon_token: recv.colon_token,
                        },
                        (Some(_), Some(_)) => FnReceiver::Slf {
                            ty: *recv.ty.clone(),
                            token: recv.self_token,
                            mutable: true,
                            reference: true,
                            lifetime: recv.lifetime().cloned(),
                            colon_token: recv.colon_token,
                        },
                    };

                    s.receiver = receiver;
                }
                syn::FnArg::Typed(t) => {
                    let param = FunctionParam::from_pat_type(t)?;
                    s.params.push(param);
                }
            }
        }

        Ok(s)
    }

    /// Converts `Self` back to a punctuated sequence of `syn::FnArg`, with all inner attributes stripped.
    pub fn to_punctuated(self) -> syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> {
        let mut res = Vec::<syn::FnArg>::new();

        match self.receiver {
            FnReceiver::None => (),
            FnReceiver::Slf {
                ty,
                token,
                mutable,
                reference,
                lifetime,
                colon_token,
            } => {
                res.push(syn::FnArg::Receiver(syn::Receiver {
                    attrs: Vec::new(),
                    reference: if reference {
                        Some((Default::default(), lifetime))
                    } else {
                        None
                    },
                    mutability: if mutable {
                        Some(Default::default())
                    } else {
                        None
                    },
                    self_token: token,
                    colon_token,
                    ty: Box::new(ty),
                }));
            }
        }

        for param in self.params {
            let pat = param.pat;
            let ty = param.ty;

            let arg = syn::FnArg::Typed(syn::PatType {
                attrs: Vec::new(),
                pat: Box::new(pat),
                colon_token: Default::default(),
                ty: Box::new(ty),
            });

            res.push(arg);
        }

        res.into_iter().map(|x| x).collect()
    }
}

impl FunctionParam {
    /// Parse a type ascription pattern into `Self`.
    pub fn from_pat_type(punct: syn::PatType) -> Result<Self, syn::Error> {
        let pat = &punct.pat;
        let ty = &punct.ty;
        let mut default_value = ParamAttr::None;

        // look for default attr
        if punct.attrs.len() > 0 {
            for attr in &punct.attrs {
                if attr.path().is_ident(crate::DEFAULT_ATTR) {
                    let meta = attr.meta.clone();

                    match meta {
                        syn::Meta::Path(_) => default_value = ParamAttr::Default,
                        syn::Meta::List(l) => {
                            let l_span = l.span();

                            let first_item = l.tokens.into_iter().next().ok_or(syn::Error::new(
                                l_span,
                                "expected at least 1 item in metalist",
                            ))?;

                            // return Err(syn::Error::new(first_item.span(), format!("first item is : {}", first_item)));

                            // match first_item {
                            //     proc_macro2::TokenTree::Group(g) => todo!(),
                            //     proc_macro2::TokenTree::Ident(id) => todo!(),
                            //     proc_macro2::TokenTree::Punct(p) => todo!(),
                            //     proc_macro2::TokenTree::Literal(l) => todo!(),
                            // }

                            // let g = if let proc_macro2::TokenTree::Group(_g) = first_item {
                            //     _g
                            // } else {
                            //     return Err(syn::Error::new(
                            //         first_item.span(),
                            //         "expected tokentree group",
                            //     ));
                            // };

                            let e: syn::Expr = syn::parse2(first_item.to_token_stream())?;
                            default_value = ParamAttr::Value(e);
                        }
                        syn::Meta::NameValue(nv) => {
                            let e = syn::Error::new(
                                    nv.span(),
                                    format!("name-values are not supported. Use #[{}] or #[{}(CONST_VALUE)] instead.",
                                        crate::DEFAULT_ATTR,
                                        crate::DEFAULT_ATTR
                                    ),
                                );
                            return Err(e);
                        }
                    }

                    break;
                }
            }
        }

        Ok(Self {
            pat: *pat.clone(),
            ty: *ty.clone(),
            default_value,
        })
    }
}
