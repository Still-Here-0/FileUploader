#[macro_export]
macro_rules! try_unwrap_in_place {
    ( $var:ident ) => {
        let mut $var = $var.ok_or_else(|| {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            let function_name = &name[..name.len() - 3];
            anyhow::anyhow!("No sql parameter passed to '{}' in '{}'", stringify!($var), function_name)
        })?;
    };
}

#[macro_export]
macro_rules! try_get_glob {
    ($glob:expr, $key:expr) => {
        $glob.get($key).ok_or_else(|| anyhow::anyhow!("No '{}' value in the global parameters", $key))?.clone()
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

#[macro_export]
macro_rules! impl_to_sql_value {
    ($type:ty, $variant:ident) => {
        impl ToSqlValue for $type {
            fn to_sql_value(self) -> SqlValue {
                SqlValue::$variant(self)
            }
        }
    };
}
