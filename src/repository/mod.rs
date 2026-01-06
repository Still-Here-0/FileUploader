use crate::ddb::DBLoad;
use crate::ddb::context::db_types::{SqlMultipleParameters, SqlSingleParameters, ChainReturn};
use crate::ddb::context::functions::build_insert_clause;
use crate::ddb::tables::{Sheet, SheetMetaData};
use crate::{st, try_get_glob, try_unwrap_in_place};


pub fn sheet_insert(
    mult: Option<SqlMultipleParameters>,
    sing: Option<SqlSingleParameters>, 
    glob: &SqlSingleParameters
) -> ChainReturn {
    try_unwrap_in_place!(mult);
    let sql = build_insert_clause(Sheet::TAB, &mult)?;

    Ok((
        sql,
        Some(mult.to_single()),
        Some(st!(Sheet::COL_PK))
    ))
}

pub fn sheet_meta_data_insert(
    mult: Option<SqlMultipleParameters>,
    sing: Option<SqlSingleParameters>, 
    glob: &SqlSingleParameters
) -> ChainReturn {
    try_unwrap_in_place!(mult);
    let sheet_id = try_get_glob!(glob, Sheet::COL_PK);
    mult.add_const_column(sheet_id, SheetMetaData::COL_SHEET_FK);
    let sql = build_insert_clause(SheetMetaData::TAB, &mult)?;

    Ok((
        sql,
        Some(mult.to_single()),
        None
    ))
}