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
    let http = doc.get("http").unwrap();
    let http_block = http.children().unwrap();
    for node in http_block.nodes() {
        match node.name().value() {
            "server" => {
                if let Some(server_block) = node.children() {
                    println!("{}", server_block)
                }
            }
            _ => {}
        }
    }
    let knuffel_config = knuffel::parse::<TopBlock>("example.kdl", CONFIG_KDL).is_err();
    assert!(knuffel_config);
    let kaydle_config = kaydle::serde::from_str::<TopBlock>(CONFIG_KDL).is_err();
    assert!(kaydle_config);
}
