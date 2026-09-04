use std::collections::BTreeMap;

use arbitrary::Arbitrary;
use serde::Serialize;

#[derive(Debug, Arbitrary, Serialize)]
pub enum FuzzValue {
    None,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<FuzzValue>),
    Map(BTreeMap<String, FuzzValue>),
}

#[derive(Debug, Arbitrary)]
pub struct FuzzCase {
    pub template: String,
    pub context: FuzzValue,
    #[allow(dead_code)]
    pub sandbox: bool,
}
