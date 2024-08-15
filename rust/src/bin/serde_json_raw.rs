use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct Block {
    name: String,
    age: i32,
}
fn main() {
    let n = "60";
    RawValue::from_string(n.to_string()).unwrap();
    let block = Block {
        age: 1,
        name: "haha".into(),
    };
    let x = serde_json::to_string(&block).unwrap();
    let raw_value = RawValue::from_string(x).unwrap();
    println!("{}", raw_value);
    serde_json::from_str::<String>("\"1\"").unwrap();
}
