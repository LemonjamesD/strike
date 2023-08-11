use serde_json::Value;

pub struct OpCode {
    op: usize,
    d: Option<Value>,
    s: Option<usize>,
    t: Option<String>,
}
