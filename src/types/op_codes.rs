use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct OpCode {
    pub(crate) op: usize,
    pub(crate) d: Option<Value>,
    pub(crate) s: Option<usize>,
    pub(crate) t: Option<String>,
}
