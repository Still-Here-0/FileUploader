use quote::{ToTokens, quote};
use syn::{Ident, LitStr, Token, Type, parse::{Parse, ParseStream}};

use crate::helper;

pub struct DBLoadInput {
    table_type: Type,
    table_name: LitStr,
    cols: Vec<Column>,
}

impl Parse for DBLoadInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let table_name = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut cols = vec![];
        while !input.is_empty() {
            cols.push(input.parse()?);
            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }

        Ok(DBLoadInput { table_type, table_name, cols })
    }
}

impl ToTokens for DBLoadInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DBLoadInput { table_type, table_name, cols } = &self;

        let count = cols.len();
        let count_lit = syn::Index::from(count);

        let arr_cols = cols.iter().map(|col| {
            let reference = &col.reference;
            quote!(Self::#reference)
        });

        let get_cols = cols.iter().map(|col| {
            let reference = &col.reference;
            if col.optional {
                return quote!(row.try_get_by_name(Self::#reference)?);
            }

            quote!(row.try_get_by_name(Self::#reference)?.unwrap())
        });

        tokens.extend(quote! {
            impl DBLoad for #table_type {
                const LEN: usize = #count;
                const TAB: &'static str = #table_name;
                const COLS: &'static [&'static str] = &[ #( #arr_cols ),* ];

                fn from_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>> {
                    let mut row_stream = stream.into_row_stream();
                    let mut result = Vec::new();

                    Box::pin(async move {
                        while let Some(row) = row_stream.next().await.transpose()? as Option<tiberius::Row> {
                            result.push(Self::db_new(
                                #( #get_cols ),*
                            ));
                        }
                        Ok(result)
                    })
                }
            }
        })
    }
}

struct Column {
    reference: Ident,
    optional: bool
}

impl Parse for Column {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok( Self {
            reference: input.parse()?,
            optional: input.parse::<Token![?]>().is_ok()
        })
    }
}