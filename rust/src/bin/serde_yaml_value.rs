use serde::{Deserialize, Serialize};
use serde_yaml::Value;

const CONFIG: &'static str = include_str!("../../config.yaml");
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct HostConfig {
    enable: bool,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Settings {
    unique_name: String,
    inbound_type: String,
    inbound_settings: Value,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct GlobalConfig {
    services: Vec<Settings>,
}
fn main() {
    let c: GlobalConfig = serde_yaml::from_str(CONFIG).unwrap();
    for s in c.services {
        let x: HostConfig = serde_yaml::from_value(s.inbound_settings).unwrap();
        println!("enable {}", x.enable)
    }
}
