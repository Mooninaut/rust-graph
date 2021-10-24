use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Text(String),
    Unsigned64(u64),
    Signed64(i64),
    Float64(f64),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}
