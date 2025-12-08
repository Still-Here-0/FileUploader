#[macro_export]
macro_rules! unwrap_in_place {
    ( $($var:ident),+ $(,)? ) => {
        $(
            let $var = $var.unwrap();
        )+
    };
}

#[macro_export]
macro_rules! from_tiberius_value {
    ($ty:ty) => {
        impl TiberiusCoversion for $ty {
            type SqlType<'c> = $ty;

            #[inline]
            fn convert<'c>(v: Self::SqlType<'c>) -> $ty {
                v
            }
        }
    };
}
