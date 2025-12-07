#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
extern crate proc_macro;

mod helper;

mod python;

/// # Python inspired tuple comprehension
/// 
/// ## Description
/// In python it is easy to generate a iterable from another 
/// iterable by using comprehensions. Here we are going to
/// implement a tuple comprehension to generate a rust iter
/// 
/// xs -> iter
/// x  -> item (number in example)
/// 
/// ## Python implementation:
/// ( x + 2 for x in xs if x > 0 )
/// 
/// ## Raw rust implementation:
/// core::iter::IntoIterator::into_iter(xs)
///     .flat_map(|x| { (x > 0).then(|| x + 2) })
/// 
/// ## Implementation:
/// {map}            {for}                                  ({if})*
/// {map_expression} 'for' {pattern} 'in' {iter_expression} ('if' bool_expression)*
#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let meta = parse_macro_input!(input as python::Comp);
    quote! { #meta }.into()
}

/// # Python inspired unpacking
/// 
/// ## Description
/// In python it is easy to pass a iterable as parameters to
/// a funciton by using the "*" operator. Here we are going to
/// implement that same logic for tuples
/// 
/// func  -> Callable[int, int]
/// tuple -> [int, int]
/// 
/// ## Python implementation:
/// func(*tuple)
/// 
/// ## Raw rust emplementation:
/// func(tuple.1, tuple.2)
#[proc_macro]
pub fn unpack(input: TokenStream) -> proc_macro::TokenStream {
    todo!()
}

mod ddb;

/// # Macro that implements DBLoad into any struct
/// 
/// TableStruct -> Is a struct that implements db_new, and its db columns names
/// col1 -> Is a const str declared in TableStruct that represents the name of a column in sql where that value can't be null
/// col1 -> Is a const str declared in TableStruct that represents the name of a column in sql where that value can be null
/// 
/// impl_dbload![TableStruct, "TABLE_NAME", col1, col2?]
/// 
#[proc_macro]
pub fn dbload(input: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(input as ddb::DBLoadInput);
    quote! { #meta }.into()
}
