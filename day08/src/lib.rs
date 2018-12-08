extern crate util;
extern crate rand;

use rand::prelude::random;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

type NodeId = i32;
type Tree = HashMap<NodeId, Node>;

#[derive(Debug, Clone)]
struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    children: HashSet<NodeId>,
    metadata: Vec<i32>,
}

pub fn metadata_sum(s: &str) -> i32 {
    let tree = build_tree(&s);
    let mut total = 0;
    for n in tree.values() {
        let s: i32 = n.metadata.iter().sum();
        total += s;
    }
    total
}

fn build_tree(s: &str) -> Tree {
    let mut nums: VecDeque<NodeId> = s.split_whitespace().flat_map(|x| x.parse()).collect();
    let mut tree: Tree = HashMap::new();
    add_node(None, &mut tree, &mut nums);
    tree
}

fn add_node(parent_id: Option<NodeId>, tree: &mut Tree, nums: &mut VecDeque<NodeId>) -> NodeId {
    let child_cnt = nums.pop_front().expect("Child count") as usize;
    let meta_cnt: usize = nums.pop_front().expect("Metadata count") as usize;
    let node_id: i32 = random();
    let mut node = Node {
        id: node_id.clone(),
        parent: parent_id,
        children: HashSet::new(),
        metadata: Vec::new(),
    };
    for _ in 0..child_cnt {
        let child_id = add_node(node.parent, tree, nums);
        node.children.insert(child_id);
    }
    node.metadata = nums.iter().take(meta_cnt).cloned().collect();
    nums.drain(..meta_cnt);
    tree.insert(node_id, node);
    node_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_node() {
        let input = "0 3 10 11 12";
        assert_eq!(metadata_sum(&input), 33);
    }

    #[test]
    fn part1_example() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(metadata_sum(&input), 138);
    }
}
