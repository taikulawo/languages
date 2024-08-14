use serde_json::value::RawValue;

fn main() {
    let r = "1".to_string();
    let raw = RawValue::from_string(r).unwrap();
    raw.get().parse::<u32>().unwrap();

    let r = "\"1\"".to_string();
    let raw = RawValue::from_string(r).unwrap();
    raw.get().parse::<String>().unwrap();
}
