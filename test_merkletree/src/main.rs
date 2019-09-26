/*
merkletree

coded by Jongwhan Lee

2019. 9.27.

<= Future is Now =>
*/

use blake2::{Blake2b, Digest};
use bytes;
use bytes::Bytes;
use log::debug;
use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct Node {
    value: Bytes,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
#[derive(Debug, Default, Clone)]
struct Merkletree {
    pub nodes: Vec<Node>,
}

impl Merkletree {
    pub fn add(&mut self, n: String) {
        let encoded: Vec<u8> = bincode::serialize(&n).unwrap();
        let mut hasher = Blake2b::new();
        hasher.input(encoded);
        let hash: Bytes = Bytes::from(&hasher.result()[..]);

        debug!("add {} {}", n, hex::encode(&hash));
        let mut newone = Node::default();
        newone.value = hash;
        self.nodes.push(newone);
    }

    // compute merkletree also making topology
    pub fn compute(&mut self, nodes: &Vec<Node>) -> Vec<Node> {
        debug!("compute {}", nodes.len());
        if nodes.len() == 1 {
            return nodes.clone();
        }
        let n = (nodes.len() + 1) / 2;

        let mut tmp: Vec<Node> = vec![];
        for i in 0..n {
            let a = &self.nodes[i];
            let mut j = i + 1;
            if j > nodes.len() - 1 {
                j = nodes.len() - 1;
            }
            debug!("{} {}", i, j);
            let b = &nodes[j];
            let mut hasher = Blake2b::new();
            hasher.input(&a.value);
            hasher.input(&b.value);
            let hash: Bytes = Bytes::from(&hasher.result()[..]);
            let mut newone = Node::default();
            newone.left = Some(Box::new(a.clone()));
            newone.right = Some(Box::new(b.clone()));
            newone.value = hash;
            tmp.push(newone);
        }
        return self.compute(&tmp);
    }

    // shows all
    pub fn show(node: &Node) {
        debug!("node= {}", hex::encode(&node.value));
        if node.left.is_some() {
            Merkletree::show(&node.left.as_ref().unwrap());
        }
        if node.right.is_some() {
            Merkletree::show(&node.right.as_ref().unwrap());
        }
    }
}

fn main() {
    env_logger::init();
    let mut tree = Merkletree::default();
    tree.add("apple".into());
    tree.add("pear".into());
    tree.add("strawberry".into());
    let root_hash = tree.compute(&tree.nodes.clone());
    println!("merkletree hash = {}", hex::encode(&root_hash[0].value));
    Merkletree::show(&root_hash[0]);
}
