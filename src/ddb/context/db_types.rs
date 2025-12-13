use std::{any, collections::HashMap, fmt::format};

use chrono::{NaiveDate, NaiveDateTime};

use super::utils::escape;

#[derive(Debug, Clone)]
pub struct GenericTable {
    name: String,
    columns: Vec<GenericColumn>
}

#[derive(Debug, Clone)]
pub struct GenericColumn {
    name: String,
    typing: tiberius::ColumnType,
    optional: bool,
}


pub trait ToGenericColumnType {
    fn to_generic_column_type(&self) -> anyhow::Result<tiberius::ColumnType>;
}

impl ToGenericColumnType for String {
    fn to_generic_column_type(&self) -> anyhow::Result<tiberius::ColumnType> {
        match self.trim().to_uppercase().as_str() {
            // Integer types
            "INT" | "INT4" | "INTEGER" => Ok(tiberius::ColumnType::Int4),
            "BIGINT" | "INT8" => Ok(tiberius::ColumnType::Int8),
            "SMALLINT" | "INT2" => Ok(tiberius::ColumnType::Int2),
            "TINYINT" | "INT1" => Ok(tiberius::ColumnType::Int1),
            
            // Float types
            "FLOAT" | "FLOAT8" | "DOUBLE" => Ok(tiberius::ColumnType::Float8),
            "REAL" | "FLOAT4" => Ok(tiberius::ColumnType::Float4),
            "DECIMAL" | "NUMERIC" => Ok(tiberius::ColumnType::Decimaln),
            
            // Boolean
            "BIT" | "BOOL" | "BOOLEAN" => Ok(tiberius::ColumnType::Bit),
            
            // String types
            "VARCHAR" | "VARCHAR(MAX)" | "CHAR" => Ok(tiberius::ColumnType::BigVarChar),
            "NVARCHAR" | "NVARCHAR(MAX)" | "NCHAR" | "TEXT" => Ok(tiberius::ColumnType::NVarchar),
            
            // Date/Time types
            "DATE" => Ok(tiberius::ColumnType::Daten),
            "TIME" => Ok(tiberius::ColumnType::Timen),
            "DATETIME" | "DATETIME2" => Ok(tiberius::ColumnType::Datetime2),
            "DATETIMEOFFSET" => Ok(tiberius::ColumnType::DatetimeOffsetn),
            
            // Binary types
            "BINARY" | "VARBINARY" | "VARBINARY(MAX)" => Ok(tiberius::ColumnType::BigVarBin),
            
            // UUID
            "UNIQUEIDENTIFIER" => Ok(tiberius::ColumnType::Guid),
            
            // XML
            "XML" => Ok(tiberius::ColumnType::Xml),
            
            _ => Err(anyhow::anyhow!(
                "Unknown SQL type '{}'. Please see './src/ddb/context/db_types::ToGenericColumnType' before adding types",
                self
            ))
        }
    }
}

pub type SqlParameters = HashMap<String, SqlValue>;

#[derive(Debug, Clone)]
pub enum SqlValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    Str(String, bool),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    IntList(Vec<i32>),
    FloatList(Vec<f64>),
    StrList(Vec<String>),
}

impl SqlValue {
    pub fn to_string(&self) -> String {
        match self {
            SqlValue::Int(v) => 
                format!("{v}"),

            SqlValue::Float(v) => format!("{v}"),

            SqlValue::Bool(v) => format!("{}", if *v { 1 } else { 0 }),

            SqlValue::Str(v, _) => format!("{}", escape(v)),

            SqlValue::Date(v) => format!("{v}"),

            SqlValue::DateTime(v) => format!("{v}"),

            SqlValue::IntList(list) =>
                    list.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),

            SqlValue::FloatList(list) =>
                    list.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),

            SqlValue::StrList(list) =>
                    list.iter()
                        .map(|v| format!("'{}'", escape(v)))
                        .collect::<Vec<_>>()
                        .join(", "),
        }
    }

    pub fn to_sql_where(&self) -> String {
        match self {
            SqlValue::IntList(_) | SqlValue::FloatList(_) | SqlValue::StrList(_) => 
                format!("IN ({})", self.to_string()),

            SqlValue::Str(v, is_like) => {
                match is_like {
                    true => format!("like '{}'", escape(v)),
                    false => format!("= '{}'", escape(v)),
                }
            }

            SqlValue::Date(_) | SqlValue::DateTime(_) => 
                format!("= '{}'", self.to_string()),

            _ => format!("= {}", self.to_string()),
        }
    }
}


pub type ChainReturn = (String, Option<String>);
pub type ChainedExec<'a> = Vec<&'a dyn Fn(&SqlParameters) -> ChainReturn>;
