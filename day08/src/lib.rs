extern crate util;
extern crate rand;

use rand::prelude::random;
use std::collections::HashMap;
use std::collections::VecDeque;

type NodeId = i32;
type Tree = HashMap<NodeId, Node>;

#[derive(Debug, Clone)]
struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    metadata: Vec<i32>,
}

impl Node {
    fn compute_value(&self, tree: &Tree) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let mut total = 0;
            for v in self.metadata.clone() {
                let child_id = self.children.get((v as usize) - 1);
                total +=
                    match child_id {
                        None => 0,
                        Some(x) =>
                        {
                            let n: &Node = tree.get(&x).unwrap();
                            n.compute_value(tree)
                        },
                    }
            }
            total
        }
    }
}

pub fn metadata_sum(s: &str) -> i32 {
    let (_root_id, tree) = build_tree(&s);
    let mut total = 0;
    for n in tree.values() {
        let s: i32 = n.metadata.iter().sum();
        total += s;
    }
    total
}

pub fn compute_root_value(s: &str) -> i32 {
    let (root_id, tree) = build_tree(&s);
    let root = tree.get(&root_id).unwrap();
    root.compute_value(&tree)
}

fn build_tree(s: &str) -> (NodeId, Tree) {
    let mut nums: VecDeque<NodeId> = s.split_whitespace().flat_map(|x| x.parse()).collect();
    let mut tree: Tree = HashMap::new();
    let root_id = add_node(None, &mut tree, &mut nums);
    (root_id, tree)
}

fn add_node(parent_id: Option<NodeId>, tree: &mut Tree, nums: &mut VecDeque<NodeId>) -> NodeId {
    let child_cnt = nums.pop_front().expect("Child count") as usize;
    let meta_cnt: usize = nums.pop_front().expect("Metadata count") as usize;
    let node_id: i32 = random();
    let mut node = Node {
        id: node_id.clone(),
        parent: parent_id,
        children: Vec::new(),
        metadata: Vec::new(),
    };
    for _ in 0..child_cnt {
        let child_id = add_node(node.parent, tree, nums);
        node.children.push(child_id);
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

    #[test]
    fn one_node_value() {
        let input = "0 3 10 11 12";
        assert_eq!(compute_root_value(&input), 33);
    }

    #[test]
    fn no_child_value() {
        let input = "1 1 0 1 99 2";
        assert_eq!(compute_root_value(&input), 0);
    }

    #[test]
    fn part2_example() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(compute_root_value(&input), 66);
    }
}
