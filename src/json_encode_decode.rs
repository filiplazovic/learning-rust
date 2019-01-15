use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub parent: u32,
    pub sibling: Option<u32>
}

pub type Tree = HashMap<u32, Node>;

pub fn do_stuff() {
    let mut hm: HashMap<u32, Tree> = HashMap::new();
    let mut tree: Tree = HashMap::new();
    let node = Node {
        parent: 1,
        sibling: Some(1u32)
    };

    tree.insert(1u32, node);
    hm.insert(1u32, tree);

    let serialized = serde_json::to_string(&hm).unwrap();
    println!("serialized = {:?}", serialized);
}
