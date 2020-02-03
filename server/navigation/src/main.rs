#[macro_use]
extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use nickel::status::StatusCode;
use nickel::{HttpRouter, JsonBody, MediaType, Nickel};
#[derive(Serialize, Deserialize)]
pub struct NavNode {
    pub description: String,
    pub link: String,
    pub subnodes: Vec<NavNode>,
}

impl NavNode {
    pub fn new(link: String, description: String) -> Self {
        return NavNode {
            description: description,
            link: link,
            subnodes: vec![],
        };
    }
}

fn GetTopNodes() -> Vec<NavNode> {
    let node1 = NavNode::new("blah".to_string(), "desc".to_string());

    let nodes = vec![node1];

    return nodes;
}

fn Serialize(obj_to_serialize: &NavNode) -> String {
    let j = serde_json::to_string(obj_to_serialize).unwrap();

    return format!("{}", j);
}

fn main() {
    let mut server = Nickel::new();
    server.get(
        "**",
        middleware! { |req|

            let nodes = GetTopNodes();
            serde_json::to_value(nodes).map_err(|e| (StatusCode::InternalServerError, e))

        },
    );
    server.listen("127.0.0.1:6767");
}
