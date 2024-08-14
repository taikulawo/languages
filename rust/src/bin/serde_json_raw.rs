use serde_json::value::RawValue;

fn main() {
    let n = "60";
    RawValue::from_string(n.to_string()).unwrap();
}
