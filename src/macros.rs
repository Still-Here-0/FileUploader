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
    ($ty1:ty, ty2:ty) => {
        impl TiberiusCoversion for $ty1 {
            type SqlType<'c> = $ty2;

            #[inline]
            fn convert<'c>(v: Self::SqlType<'c>) -> $ty2 {
                v
            }
        }
    };
}

#[macro_export]
macro_rules! st {
    ($s:expr) => {
        ::std::string::String::from($s)
    };
}
