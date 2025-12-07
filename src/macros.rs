#[macro_export]
macro_rules! unwrap {
    ( $($var:ident),+ $(,)? ) => {
        $(
            let $var = $var.unwrap();
        )+
    };
}
