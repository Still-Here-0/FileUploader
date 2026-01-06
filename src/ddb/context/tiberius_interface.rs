use std::array;

use chrono::{ NaiveDate, NaiveDateTime, NaiveTime };
use tiberius::FromSql;

use crate::from_tiberius_value;
use super::db_types::SqlValue;

pub trait TiberiusCoversion: Sized {
    type SqlType<'c>: FromSql<'c>;

    fn convert<'c>(v: Self::SqlType<'c>) -> Self;
}

/* #region From reference to owned */

impl TiberiusCoversion for String {
    type SqlType<'c> = &'c str;

    fn convert<'c>(v: Self::SqlType<'c>) -> Self {
        v.to_string()
    }
}

impl TiberiusCoversion for Vec<u8> {
    type SqlType<'c> = &'c [u8];

    fn convert<'c>(v: Self::SqlType<'c>) -> Self {
        v.to_vec()
    }
}

/* #endregion */

// Owned types
from_tiberius_value!(u8);
from_tiberius_value!(i16);
from_tiberius_value!(i32);
from_tiberius_value!(i64);
from_tiberius_value!(f32);
from_tiberius_value!(f64);
from_tiberius_value!(bool);
from_tiberius_value!(NaiveDate);
from_tiberius_value!(NaiveTime);
from_tiberius_value!(NaiveDateTime);

pub trait FromOwnedSql {
    fn try_get_by_index<T>(&self, idx: usize) -> anyhow::Result<Option<T>>
    where
        T: TiberiusCoversion;

    fn try_get_by_name<T>(&self, idx: &str) -> anyhow::Result<Option<T>>
    where
        T: TiberiusCoversion;
}

impl FromOwnedSql for tiberius::Row {
    fn try_get_by_index<T>(&self, idx: usize) -> anyhow::Result<Option<T>>
    where
        T: TiberiusCoversion
    {
        let opt_unowened: Option<<T as TiberiusCoversion>::SqlType<'_>> = self.try_get(idx)?;
        let opt_owened = opt_unowened.map(T::convert);

        Ok(opt_owened)
    }

    fn try_get_by_name<T>(&self, name: &str) -> anyhow::Result<Option<T>>
    where
        T: TiberiusCoversion
    {
        let opt_unowened: Option<<T as TiberiusCoversion>::SqlType<'_>> = self.try_get(name)?;
        let opt_owened = opt_unowened.map(T::convert);

        Ok(opt_owened)
    }
}
