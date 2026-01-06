mod sql_parameters;
pub use sql_parameters::*;

mod sql_value;
pub use sql_value::*;

mod chain;
pub use chain::*;


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
            
            // Boolean types
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
            
            // UUID types // TODO: will this be used?
            "UNIQUEIDENTIFIER" => Ok(tiberius::ColumnType::Guid),
            
            // XML type // TODO: will this be used?
            "XML" => Ok(tiberius::ColumnType::Xml),
            
            _ => Err(anyhow::anyhow!(
                "Unknown SQL type '{}'. Please see './src/ddb/context/db_types::ToGenericColumnType' before adding types",
                self
            ))
        }
    }
}
