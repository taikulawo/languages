use kdl::KdlDocument;
use serde::{Deserialize, Serialize};
const CONFIG_KDL: &str = include_str!("../../config/nginx.kdl");
const INCLUDED: &str = include_str!("../../config/include.kdl");
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
    let mut doc: KdlDocument = CONFIG_KDL.parse().expect("failed to parse KDL");
    let http = doc.get_mut("http").unwrap();
    let http_block = http.children_mut().as_mut().unwrap();
    let kdl_nodes = http_block.nodes_mut();
    while kdl_nodes.len() > 0 {
        let node = kdl_nodes.remove(0);
        match node.name().value() {
            "server" => {
                if let Some(server_block) = node.children() {
                    println!("{}", server_block)
                }
            }
            "include" => {
                let included_doc: KdlDocument = INCLUDED.parse().expect("failed to parse KDL");
                let new_nodes = included_doc.nodes();
                kdl_nodes.splice(0..0, new_nodes.iter().cloned());
            }
            _ => {}
        }
    }
    let knuffel_config = knuffel::parse::<TopBlock>("example.kdl", CONFIG_KDL).is_err();
    assert!(knuffel_config);
    let kaydle_config = kaydle::serde::from_str::<TopBlock>(CONFIG_KDL).is_err();
    assert!(kaydle_config);
}
