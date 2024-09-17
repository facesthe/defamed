//! Function param stuff

use core::panic;
use std::fmt::Debug;

use quote::{quote, ToTokens};
use syn::spanned::Spanned;

use crate::traits::ToMacroPattern;

/// Parsed function parameters
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
    attrs: Vec<syn::Attribute>,
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

/// Permutation of positional and named parameters
#[derive(Clone)]
pub enum PermutedParam {
    Positional(FunctionParam),
    Named(FunctionParam),

    // default parameter that is passed as an argument
    DefaultUsed(FunctionParam),
    // default parameter that is left blank
    DefaultUnused(FunctionParam),
}

impl Debug for FunctionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionParam")
            .field("pat", &self.pat.to_token_stream().to_string())
            .field("ty", &self.ty.to_token_stream().to_string())
            .field("default_value", &self.default_value)
            .finish()
    }
}

impl Debug for ParamAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Default => write!(f, "Default"),
            Self::Value(arg0) => write!(f, "Value({})", arg0.to_token_stream()),
        }
    }
}

impl Debug for PermutedParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positional(arg0) => f.debug_tuple("Positional").field(arg0).finish(),
            Self::Named(arg0) => f.debug_tuple("Named").field(arg0).finish(),
            Self::DefaultUsed(arg0) => f.debug_tuple("DefaultUsed").field(arg0).finish(),
            Self::DefaultUnused(arg0) => f.debug_tuple("DefaultUnused").field(arg0).finish(),
        }
    }
}

impl ToMacroPattern for PermutedParam {
    fn to_macro_pattern(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            PermutedParam::Positional(inner) => {
                let pat = &inner.pat;
                let val = syn::Ident::new(&format!("{}_val", pat.to_token_stream()), pat.span());
                Some(quote! {$#val: expr})
            }
            PermutedParam::Named(inner) | PermutedParam::DefaultUsed(inner) => {
                let pat = &inner.pat;
                let val = syn::Ident::new(&format!("{}_val", pat.to_token_stream()), pat.span());
                Some(quote! {#pat = $#val: expr})
            }
            PermutedParam::DefaultUnused(_) => None,
        }
    }

    fn to_func_call_pattern(&self) -> proc_macro2::TokenStream {
        match self {
            PermutedParam::Positional(inner)
            | PermutedParam::Named(inner)
            | PermutedParam::DefaultUsed(inner) => {
                let pat = &inner.pat;
                let val = syn::Ident::new(&format!("{}_val", pat.to_token_stream()), pat.span());

                quote! {$#val}
            }

            PermutedParam::DefaultUnused(inner) => match &inner.default_value {
                ParamAttr::None => unimplemented!("invalid inner value"),
                ParamAttr::Default => quote! {core::default::Default::default()},
                ParamAttr::Value(v) => quote! {#v},
            },
        }
    }
}

// simple string matching
impl PartialEq for FunctionParam {
    fn eq(&self, other: &Self) -> bool {
        self.pat.to_token_stream().to_string() == other.pat.to_token_stream().to_string()
        // && self.ty == other.ty && self.default_value == other.default_value
    }
}

/// Compares the inner values, since they are all the same type
impl PartialEq for PermutedParam {
    fn eq(&self, other: &Self) -> bool {
        let inner = match self {
            PermutedParam::Positional(_i) => _i,
            PermutedParam::Named(_i) => _i,
            PermutedParam::DefaultUsed(_i) => _i,
            PermutedParam::DefaultUnused(_i) => _i,
        };

        let othr = match other {
            PermutedParam::Positional(_i) => _i,
            PermutedParam::Named(_i) => _i,
            PermutedParam::DefaultUsed(_i) => _i,
            PermutedParam::DefaultUnused(_i) => _i,
        };

        inner == othr
    }
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

    /// Converts `Self` back to a punctuated sequence of `syn::FnArg`, with all matching inner attributes stripped.
    pub fn to_punctuated(&self) -> syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> {
        let mut res = Vec::<syn::FnArg>::new();

        match &self.receiver {
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
                    reference: if *reference {
                        Some((Default::default(), lifetime.clone()))
                    } else {
                        None
                    },
                    mutability: if *mutable {
                        Some(Default::default())
                    } else {
                        None
                    },
                    self_token: *token,
                    colon_token: *colon_token,
                    ty: Box::new(ty.clone()),
                }));
            }
        }

        for param in &self.params {
            let pat = param.pat.to_owned();
            let ty = param.ty.to_owned();
            let s_attrs = param
                .attrs
                .iter()
                .filter(|a| !a.path().is_ident(crate::DEFAULT_HELPER_ATTR))
                .cloned()
                .collect::<Vec<_>>();

            let arg = syn::FnArg::Typed(syn::PatType {
                attrs: s_attrs,
                pat: Box::new(pat),
                colon_token: Default::default(),
                ty: Box::new(ty),
            });

            res.push(arg);
        }

        res.into_iter().collect()
    }

    /// Checks if the token sequence adheres to the following:
    /// - Default parameters must be at the end of the sequence
    ///
    /// TODO: write a test for this
    pub fn first_invalid_param(&self) -> Option<&FunctionParam> {
        let mut iter = self.params.iter();

        // advance to first default parameter
        let first_default = loop {
            if let Some(param) = iter.next() {
                match param.default_value {
                    ParamAttr::None => (),
                    _ => break param,
                }
            } else {
                return None;
            }
        };

        match iter.all(|item| !matches!(item.default_value, ParamAttr::None)) {
            true => None,
            false => Some(first_default),
        }
    }

    /// Generate all permutations of positional and named parameters.
    ///
    /// The following rules are followed:
    /// - Positional parameters come first
    /// - Remaining named parameters come after positional parameters, in all possible permutations
    /// - Default used parameters are next, in all possible permutations
    /// - Default unused parameters are last, without permutations
    /// - Positional default parameters are really last, like for real
    pub fn permute_params(&self) -> Vec<Vec<PermutedParam>> {
        let required_params = self
            .params
            .iter()
            .take_while(|p| matches!(p.default_value, ParamAttr::None))
            .cloned()
            .collect::<Vec<_>>();

        let default_params = self
            .params
            .iter()
            .skip(required_params.len())
            .cloned()
            .collect::<Vec<_>>();

        let named_permute = (0..=required_params.len())
            .flat_map(|idx| {
                // let opp_idx = required_params.len() - i;
                let (positional, named) = required_params.split_at(idx);

                let positional = positional
                    .iter()
                    .map(|p| PermutedParam::Positional(p.to_owned()))
                    .collect::<Vec<_>>();
                let permute_slice = Self::permute_named(named);

                permute_slice
                    .iter()
                    .map(|named_seq| [positional.as_slice(), named_seq.as_slice()].concat())
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<Vec<_>>();

        let default_permute = Self::permute_default(&default_params);
        let default_positional_permute = Self::permute_positional_default(&default_params);

        // last element in named permutation matrix contains all positional parameters
        let all_positional = match named_permute.last() {
            Some(base) => {
                // sanity check
                assert!(base
                    .iter()
                    .all(|item| matches!(item, PermutedParam::Positional(_))));

                default_positional_permute
                    .into_iter()
                    .map(|seq| [base.as_slice(), seq.as_slice()].concat())
                    .collect()
            }
            None => default_positional_permute,
        };

        let inter = match (named_permute.len(), default_permute.len()) {
            (0, 0) => vec![],
            (0, _) => default_permute,
            (_, 0) => named_permute,
            (_, _) => named_permute
                .iter()
                .flat_map(|np| {
                    default_permute
                        .iter()
                        .map(|dp| [np.as_slice(), dp.as_slice()].concat())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        };

        match (inter.len(), all_positional.len()) {
            (0, 0) => vec![],
            (0, _) => all_positional,
            (_, 0) => inter,
            (_, _) => [inter, all_positional].concat(),
        }
    }

    /// Perform permutation of all items in slice.
    /// All items will be of the [PermutedParam::Named] variant
    fn permute_named(named: &[FunctionParam]) -> Vec<Vec<PermutedParam>> {
        if !named
            .iter()
            .all(|n| matches!(n.default_value, ParamAttr::None))
        {
            panic!("All items in slice must not have default values");
        }

        let permutations = permute::permutations_of(named);

        permutations
            .into_iter()
            .map(|single_perm| {
                single_perm
                    .into_iter()
                    .map(|item| PermutedParam::Named(item.to_owned()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    /// Perform permutations for default parameters. All permuted values are named.
    ///
    /// Each item in the slice must have a default value.
    /// Additionally, default params can be used or unused. These are also permuted as well.
    fn permute_default(defaults: &[FunctionParam]) -> Vec<Vec<PermutedParam>> {
        if !defaults
            .iter()
            .all(|n| !matches!(n.default_value, ParamAttr::None))
        {
            panic!("All items in slice must have default values");
        }

        let base_permute = (0..(1 << defaults.len()))
            .map(|num| {
                let seq = defaults
                    .iter()
                    .enumerate()
                    .map(|(pos, item)| {
                        // if bit set, it is used
                        if (num >> pos) & 1 != 0 {
                            PermutedParam::DefaultUsed(item.to_owned())
                        } else {
                            PermutedParam::DefaultUnused(item.to_owned())
                        }
                    })
                    .collect::<Vec<_>>();

                seq
            })
            .collect::<Vec<_>>();

        let res = base_permute
            .into_iter()
            .flat_map(|seq| {
                let (used, unused) = Self::split_defaults(seq);

                let mut used_permute = permute::permute(used);

                for item in &mut used_permute {
                    item.extend_from_slice(&unused);
                }

                used_permute.into_iter()
            })
            .collect::<Vec<_>>();

        res.into_iter().filter(|item| !item.is_empty()).collect()

        // res
    }

    /// Permute positional parameters for default values.
    ///
    /// This extends the special case where all preceding (non default) parameters
    /// are used as positional parameters.
    fn permute_positional_default(defaults: &[FunctionParam]) -> Vec<Vec<PermutedParam>> {
        let res = (1..=defaults.len())
            .flat_map(|idx| {
                let (positional, named) = defaults.split_at(idx);
                let pos_params = positional
                    .iter()
                    .map(|p| PermutedParam::Positional(p.to_owned()))
                    .collect::<Vec<_>>();

                let inter = match named.len() {
                    0 => vec![pos_params],
                    _ => {
                        let named_permute = Self::permute_default(named);

                        named_permute
                            .into_iter()
                            .map(move |named_seq| [pos_params.clone(), named_seq].concat())
                            .collect::<Vec<_>>()
                    }
                };

                inter.into_iter()
            })
            .collect::<Vec<_>>();

        res
    }

    /// Split the default parameters into default(used) and default(unused) parameters.
    fn split_defaults(defaults: Vec<PermutedParam>) -> (Vec<PermutedParam>, Vec<PermutedParam>) {
        let res: (Vec<_>, Vec<_>) = defaults.into_iter().partition(|def| match def {
            PermutedParam::DefaultUsed(_) => true,
            PermutedParam::DefaultUnused(_) => false,
            _ => panic!("unexpected variant"),
        });

        res
    }
}

impl FunctionParam {
    /// Parse a type ascription pattern into `Self`.
    pub fn from_pat_type(punct: syn::PatType) -> Result<Self, syn::Error> {
        let pat = &punct.pat;
        let ty = &punct.ty;
        let mut default_value = ParamAttr::None;

        // look for default attr
        if !punct.attrs.is_empty() {
            for attr in &punct.attrs {
                if attr.path().is_ident(crate::DEFAULT_HELPER_ATTR) {
                    let meta = attr.meta.clone();

                    match meta {
                        syn::Meta::Path(_) => default_value = ParamAttr::Default,
                        syn::Meta::List(l) => {
                            let l_span = l.span();

                            let first_item = l.tokens.into_iter().next().ok_or(syn::Error::new(
                                l_span,
                                "expected at least 1 item in metalist",
                            ))?;

                            let e: syn::Expr = syn::parse2(first_item.to_token_stream())?;
                            default_value = ParamAttr::Value(e);
                        }
                        syn::Meta::NameValue(nv) => {
                            let e = syn::Error::new(
                                    nv.span(),
                                    format!("name-values are not supported. Use #[{}] or #[{}(CONST_EXPRESSION)] instead.",
                                        crate::DEFAULT_HELPER_ATTR,
                                        crate::DEFAULT_HELPER_ATTR
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
            attrs: punct.attrs,
            default_value,
        })
    }

    pub fn inner_span(&self) -> proc_macro2::Span {
        self.pat.span()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::quote;
    use syn::{punctuated::Punctuated, token::Comma, FnArg};

    #[test]
    fn test_permute_named() {
        let tokens = vec![
            quote! { a: i32 },
            quote! { b: u8 },
            quote! { c: usize },
            quote! { d: i64 },
        ];

        let punct: Punctuated<FnArg, Comma> = tokens
            .into_iter()
            .map(|t| syn::parse2::<FnArg>(t).unwrap())
            .collect();

        let params = FunctionParams::from_punctuated(punct).unwrap();

        let permutations = FunctionParams::permute_named(&params.params);

        println!("{:#?}", permutations);

        // 0 0
        // 0 1
        // 1 0
        // 1 1
        assert_eq!(permutations.len(), 24);
    }

    #[test]
    fn test_permute_defaults() {
        let default_ident =
            syn::Ident::new(crate::DEFAULT_HELPER_ATTR, proc_macro2::Span::call_site());

        let tokens = vec![
            quote! { #[#default_ident] a: i32 },
            quote! { #[#default_ident(1)] c: u8 },
        ];

        let punct: Punctuated<FnArg, Comma> = tokens
            .into_iter()
            .map(|t| syn::parse2::<FnArg>(t).unwrap())
            .collect();

        let params = FunctionParams::from_punctuated(punct).unwrap();

        let permutations = FunctionParams::permute_default(&params.params);

        println!("{:#?}", permutations);

        // 0 0
        // 0 1
        // 1 0
        // 1 1
        // 1 1 again because used defaults have to be permuted
        assert_eq!(permutations.len(), 5);

        // empty case
        let permutations = FunctionParams::permute_default(&[]);
        println!("{:?}", permutations);
        assert_eq!(permutations.len(), 0);
    }

    #[test]
    fn test_permute_positional_defaults() {
        let default_ident =
            syn::Ident::new(crate::DEFAULT_HELPER_ATTR, proc_macro2::Span::call_site());

        let tokens = vec![
            quote! { #[#default_ident] a: i32 },
            quote! { #[#default_ident(1)] b: u8 },
            quote! { #[#default_ident] c: usize },
        ];

        let punct: Punctuated<FnArg, Comma> = tokens
            .into_iter()
            .map(|t| syn::parse2::<FnArg>(t).unwrap())
            .collect();

        let params = FunctionParams::from_punctuated(punct).unwrap();
        let permutations = FunctionParams::permute_positional_default(&params.params);

        println!("{:#?}", permutations);
        assert_eq!(permutations.len(), 8);
    }

    /// Full permutation test with positional and named parameters
    #[test]
    fn test_permute_all_positional_named() {
        let tokens = vec![
            quote! { a: i32 },
            quote! { b: u8 },
            quote! { c: usize },
            quote! { d: i64 },
        ];

        let punct: Punctuated<FnArg, Comma> = tokens
            .into_iter()
            .map(|t| syn::parse2::<FnArg>(t).unwrap())
            .collect();

        let params = FunctionParams::from_punctuated(punct).unwrap();

        let permutations = params.permute_params();

        println!("{:?}", permutations);

        // 34
        assert_eq!(permutations.len(), 34);
    }

    #[test]
    fn test_all_positional_full() {
        let default_token =
            syn::Ident::new(crate::DEFAULT_HELPER_ATTR, proc_macro2::Span::call_site());

        let tokens = vec![
            // 34 permutations for positional and named
            quote! { a: i32 },
            quote! { b: u8 },
            quote! { c: usize },
            quote! { d: i64 },
            // 5 permutations for default parameters
            // and 3 additional permutations for positional default parameters (not permuted)
            quote! { #[#default_token] e: i32 },
            quote! { #[#default_token(1)] f: u8 },
        ];

        let punct: Punctuated<FnArg, Comma> = tokens
            .into_iter()
            .map(|t| syn::parse2::<FnArg>(t).unwrap())
            .collect();

        let params = FunctionParams::from_punctuated(punct).unwrap();

        let permutations = params.permute_params();

        println!("{:#?}", permutations[0]);

        assert_eq!(permutations.len(), 34 * 5 + 3);
    }
}
