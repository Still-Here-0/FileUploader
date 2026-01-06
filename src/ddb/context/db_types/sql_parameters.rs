use std::{any, collections::{HashMap, HashSet}, marker::PhantomData};
use anyhow::Ok;

use super::SqlValue;

pub type SqlSingleParameters = HashMap<String, SqlValue>;



#[derive(Debug)]
pub struct SqlMultipleParameters {
    core: HashMap<String, usize>,
    columns: Vec<Vec<SqlValue>>
}

impl SqlMultipleParameters {
    pub fn new() -> Self {
        let core = HashMap::<String, usize>::new();
        let columns = Vec::new();

        Self { 
            core, 
            columns
        }
    }

    pub fn clear(&mut self) {
        self.core = HashMap::<String, usize>::new();
        self.columns = Vec::new();
    }

    fn populate_header(&mut self, column_names: &[&str]) {
        for (idx, column) in column_names.iter().enumerate() {
            self.core.insert(column.to_string(), idx);
            self.columns.push(Vec::new());
        }
    }

    pub fn add_line(&mut self, line_data: Vec<(&str, SqlValue)>) -> anyhow::Result<()> {
        if self.columns.len() == 0 {
            self.populate_header(
                &line_data.iter().map(|(name, _)| *name).collect::<Vec<&str>>()
            );
        }

        anyhow::ensure!(
            line_data.len() == self.len(),
            "Row must provide exactly one value per column (expected {}, got {})",
            self.len(),
            line_data.len()
        );

        let mut seen: HashSet<&str> = HashSet::with_capacity(line_data.len());

        for (line_name, value) in line_data {
            anyhow::ensure!(
                seen.insert(line_name),
                "Duplicate column name '{}' in line_data",
                line_name
            );

            let idx = self.get_column_idx(line_name)?;
            self.columns[idx].push(value);
        }

        anyhow::ensure!(
            seen.len() == self.len(),
            "Row must provide exactly one value per column"
        );

        Ok(())
    }

    pub fn add_const_column(&mut self, value: SqlValue, name: &str) {
        let idx = self.len();
        self.core.insert(name.to_string(), idx);
        self.columns.push(Vec::new());

        for _ in 0..self.hight() {
            self.columns[idx].push(value.clone());
        }
    }

    pub fn header(&self) -> Vec<String> {
        self.core.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.core.len()
    }

    pub fn hight(&self) -> usize {
        self.columns[0].len()
    }

    fn get_column_idx(&self, column_name: &str) -> anyhow::Result<usize> {
        self.core
            .get(column_name)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Column '{}' not found", column_name))
    }

    pub fn get_value(&self, column_name: &str, line_idx: usize) -> anyhow::Result<&SqlValue> {
        let column_idx = self.get_column_idx(column_name)?;

        Ok(&self.columns[column_idx][line_idx])
    }

    pub fn to_single(self) -> SqlSingleParameters {
        let mut single = SqlSingleParameters::new();


        for (key, column_idx) in self.core {
            for (line_idx, line_value) in self.columns[column_idx].iter().enumerate() {
                single.insert(format!("{key}_{line_idx}"), line_value.clone());
            }
        }

        single
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ddb::{DBLoad, context::db_types::ToSqlValue, tables::*}, st};

    #[test]
    fn test_mult_parameters() {
        let mut a = SqlMultipleParameters::new();

        for i in 0..5 {
            a.add_line(
                vec![
                    (ColumnType::COL_SQL_TYPE, SqlValue::Str(format!("TEST {i}"))),
                    (ColumnType::COL_VIEW_TYPE, SqlValue::Str(st!("TEST"))),
                ]
            ).unwrap();
        }

        // println!("{a:?}")
    }

    #[test]
    fn check_mult_to_single_conversion() {
        let mut mult = SqlMultipleParameters::new();
        mult.add_line(vec![
            ("A", 1.to_sql_value()),
            ("B", 2.to_sql_value()),
            ("C", 3.to_sql_value()),
        ]);
        mult.add_line(vec![
            ("A", 4.to_sql_value()),
            ("B", 5.to_sql_value()),
            ("C", 6.to_sql_value()),
        ]);
        mult.add_line(vec![
            ("A", 7.to_sql_value()),
            ("B", 8.to_sql_value()),
            ("C", 9.to_sql_value()),
        ]);

        let single = mult.to_single();
    }
}