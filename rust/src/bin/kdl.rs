use kdl::KdlDocument;
use serde::{Deserialize, Serialize};
const CONFIG_KDL: &str = include_str!("../../config/nginx.kdl");
#[derive(Serialize, Deserialize, Clone, Debug, Default, knuffel::Decode)]
struct LocationBlock {
    root: Option<String>,
    index: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default, knuffel::Decode)]
struct ServerBlock {
    listen: Option<u32>,
    server_name: Option<String>,
    location: Vec<(String, LocationBlock)>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default, knuffel::Decode)]
struct HttpBlock {
    server: Vec<ServerBlock>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default, knuffel::Decode)]
struct TopBlock {
    pub http: HttpBlock,
}
fn main() {
    let doc: KdlDocument = CONFIG_KDL.parse().expect("failed to parse KDL");
    println!("{}", doc);
    let http = doc.get("http").unwrap();
    if let Some(c) = http.children().cloned() {
        for h in c.into_iter() {
            println!("{}", h)
        }
    }
    let config = knuffel::parse::<TopBlock>("example.kdl", CONFIG_KDL).unwrap();
    // let config: TopBlock = kaydle::serde::from_str(CONFIG_KDL).unwrap();
}
