use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, symbol::Symbol}};

pub mod math;
pub mod io;

pub fn _eval(_owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let expr = lookup!(table, "expr");

    match expr.as_ref() {
        Data::String(_expr) => todo!(),
        d => Err(format!("Not possible to parse and evaluate a non String `{d}`"))
    }
}