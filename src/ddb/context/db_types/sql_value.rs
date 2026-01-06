use crate::{ddb::context::utils::escape, st};
use crate::impl_to_sql_value;

use tiberius::Query;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::de::value;

#[derive(Debug, Clone)]
pub enum SqlValue {
    Int1(u8),
    Int2(i16),
    Int(i32),
    Int8(i64),
    Float4(f32),
    Float(f64),
    Decimal(String),
    Bool(bool),
    Str(String),
    StrL(String),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    IntList(Vec<i32>),
    FloatList(Vec<f64>),
    StrList(Vec<String>),
    Bin(Vec<u8>),
    Guid(String),   // TODO: will this be used?
    Xml(String),    // TODO: will this be used?
    None,
}

impl SqlValue {
    pub fn to_string(&self) -> String {
        match self {
            SqlValue::Int(v) => format!("{v}"),

            SqlValue::Int8(v) => format!("{v}"),

            SqlValue::Int2(v) => format!("{v}"),

            SqlValue::Int1(v) => format!("{v}"),

            SqlValue::Float(v) => format!("{v}"),

            SqlValue::Float4(v) => format!("{v}"),

            SqlValue::Decimal(v) => format!("{v}"),

            SqlValue::Bool(v) => format!("{}", if *v { 1 } else { 0 }),

            SqlValue::Str(v) => format!("{}", escape(v)),

            SqlValue::StrL(v) => format!("{}", escape(v)),

            SqlValue::Date(v) => format!("{v}"),

            SqlValue::Time(v) => format!("{v}"),

            SqlValue::DateTime(v) => format!("{v}"),

            SqlValue::IntList(list) => list
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", "),

            SqlValue::FloatList(list) => list
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", "),

            SqlValue::StrList(list) => list
                .iter()
                .map(|v| format!("'{}'", escape(v)))
                .collect::<Vec<_>>()
                .join(", "),

            SqlValue::Bin(_) => st!("@BIN"),

            SqlValue::Guid(v) => format!("{v}"),

            SqlValue::Xml(v) => format!("{}", escape(v)),

            SqlValue::None => format!("NULL"),
        }
    }

    pub fn to_sql(&self) -> String {
        match self {
            SqlValue::IntList(_) | SqlValue::FloatList(_) | SqlValue::StrList(_) => {
                format!("({})", self.to_string())
            }

            SqlValue::Str(_)
            | SqlValue::StrL(_)
            | SqlValue::Date(_)
            | SqlValue::Time(_)
            | SqlValue::DateTime(_)
            | SqlValue::Guid(_)
            | SqlValue::Xml(_) => format!("'{}'", self.to_string()),

            _ => format!("{}", self.to_string()),
        }
    }

    pub fn tag_sql_where(&self, tag_name: &str) -> String {
        let tag = self.tag(tag_name);

        match self {
            SqlValue::IntList(_) | SqlValue::FloatList(_) | SqlValue::StrList(_) => {
                format!("IN ({tag})")
            }

            SqlValue::StrL(_) => format!("LIKE {tag}"),

            _ => format!("= {tag}"),
        }
    }

    pub fn bind_value(&self, query: &mut Query<'_>) -> anyhow::Result<()> {
        match self {
            SqlValue::Bin(value) => query.bind(value.clone()),
            
            _ => anyhow::bail!("Only binary values can be binded this way, you tried -> {self:?}")
        };

        Ok(())
    }

    pub fn tag(&self, tag_name: &str) -> String {
        match self {
            SqlValue::Bin(_) => format!("@{tag_name}"),
            
            _ => format!("@_{tag_name}")
        }
    }
}

pub trait ToSqlValue {
    fn to_sql_value(self) -> SqlValue;
}

impl_to_sql_value!(u8, Int1);
impl_to_sql_value!(i16, Int2);
impl_to_sql_value!(i32, Int);
impl_to_sql_value!(i64, Int8);
impl_to_sql_value!(f32, Float4);
impl_to_sql_value!(f64, Float);
impl_to_sql_value!(bool, Bool);
impl_to_sql_value!(String, Str);
impl_to_sql_value!(NaiveDate, Date);
impl_to_sql_value!(NaiveTime, Time);
impl_to_sql_value!(NaiveDateTime, DateTime);
impl_to_sql_value!(Vec<i32>, IntList);
impl_to_sql_value!(Vec<f64>, FloatList);
impl_to_sql_value!(Vec<String>, StrList);
impl_to_sql_value!(Vec<u8>, Bin);

impl<T: ToSqlValue> ToSqlValue for Option<T> {
    fn to_sql_value(self) -> SqlValue {
        match self {
            Some(value) => value.to_sql_value(),
            None => SqlValue::None,
        }
    }
}