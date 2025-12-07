use syn::parse::{Parse, ParseStream};


pub fn continuos_parse<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();

    while let Ok(item) = input.parse::<T>() {
        result.push(item);
    }

    result
}
