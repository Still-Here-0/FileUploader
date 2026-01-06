use std::iter::{Map, Zip};
use std::slice::Iter;

use tiberius::Query;

use super::{SqlMultipleParameters, SqlSingleParameters, SqlValue};

pub struct ChainMap<'a> {
    execs: Vec<ChainExec<'a>>,
    mults: Vec<Option<SqlMultipleParameters>>,
    sings: Vec<Option<SqlSingleParameters>>,
}

impl<'a> ChainMap<'a> {
    pub fn new() -> Self {
        Self {
            execs: vec![],
            mults: vec![], 
            sings: vec![], 
        }
    }

    pub fn push(
        &mut self,
        exec: ChainExec<'a>,
        mult: Option<SqlMultipleParameters>,
        sing: Option<SqlSingleParameters>
    ) {
        self.execs.push(exec);
        self.mults.push(mult);
        self.sings.push(sing);
    }
}

impl<'a> IntoIterator for ChainMap<'a> {
    type Item = (ChainExec<'a>, Option<SqlMultipleParameters>, Option<SqlSingleParameters>);
    type IntoIter = 
    Map<
        Zip<Zip<
                std::vec::IntoIter<ChainExec<'a>>, 
                std::vec::IntoIter<Option<SqlMultipleParameters>>
            >,
            std::vec::IntoIter<Option<SqlSingleParameters>>
        >,
        fn(((ChainExec<'a>, Option<SqlMultipleParameters>), Option<SqlSingleParameters>)) -> Self::Item
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.execs.into_iter()
            .zip(self.mults.into_iter())
            .zip(self.sings.into_iter())
            .map(|((exec, mult), sing)| (exec, mult, sing))
    }
}

pub type ChainReturn = anyhow::Result<(String, Option<SqlSingleParameters>, Option<String>)>;

pub type ChainExec<'a> = &'a (dyn Fn(
    Option<SqlMultipleParameters>,
    Option<SqlSingleParameters>, 
    &SqlSingleParameters
) -> ChainReturn + Send + Sync);
