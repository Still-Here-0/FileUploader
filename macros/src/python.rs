use syn::{ExprCall, parse::{Parse, ParseStream}, parse_macro_input};
use quote::{ToTokens, quote};

use crate::helper;

/* #region Unpaking */

pub struct Unpack(ExprCall);

impl ToTokens for Unpack {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Unpack(call) = &self;

        let func = call.clone().func;
        let mut args = call.args.clone().into_iter();
    }
}

impl Parse for Unpack {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse()
    }
}

/* #endregion */

/* #region Comp */

pub struct Comp {
    mapping: syn::Expr,
    for_clause: ForClause,
    if_conditions: Vec<IfCondition>
}

impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mapping = &self.mapping;
        let ForClause{ pattern, iter } = &self.for_clause;
        let if_conditions = &self.if_conditions;

        tokens.extend(quote! {
            ::core::iter::IntoIterator::into_iter(#iter)
                .flat_map(move |#pattern| { 
                    (#( (#if_conditions) &&)* true)
                        .then(|| #mapping) 
                })
        });
    }
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { 
            mapping: input.parse()?, 
            for_clause: input.parse()?, 
            if_conditions: helper::continuos_parse(input)
        })
    }
}

struct ForClause {
    pattern: syn::Pat,
    iter: syn::Expr
}

impl Parse for ForClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![for]>()?;
        let pattern = syn::Pat::parse_single(input)?;
        input.parse::<syn::Token![in]>()?;
        let iter = input.parse()?;

        Ok( Self { iter, pattern } )
    }
}

struct IfCondition(syn::Expr);

impl ToTokens for IfCondition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl Parse for IfCondition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![if]>()?;
        input.parse().map(Self)
    }
}

/* #endregion */

