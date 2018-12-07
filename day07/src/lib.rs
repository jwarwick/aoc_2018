extern crate util;
#[macro_use] extern crate scan_fmt;

use std::collections::HashSet;
use std::collections::HashMap;

//extern crate itertools;
//use itertools::Itertools;

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<char, HashSet<char>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {nodes: HashMap::new()}
    }

    fn add_edge(&mut self, from: char, to: char) {
        self.nodes.entry(from).or_insert(HashSet::new());
        let to_node = self.nodes.entry(to).or_insert(HashSet::new());
        to_node.insert(from);
    }

    fn topo_sort(&self) -> String {
        let mut n = self.nodes.clone();
        let mut output: String = String::new();

        while !n.is_empty() {
            let s = next_node(&mut n);
            n.remove(&s);
            for val in n.values_mut() {
                val.remove(&s);
            }
            output.push(s);
        }

        output
    }
}

fn next_node(nodes: &mut HashMap<char, HashSet<char>>) -> char {
    let mut edges: Vec<_> = nodes.iter().filter(|(_k, v)| v.is_empty()).collect();
    edges.sort_by_key(|(&k, _v)| k);
    let (c, _v) = edges.first().expect("A node with no inputs");
    **c
}

pub fn ordering(s: &str) -> String {
    let mut graph = Graph::new();
    for l in s.lines() {
        let (pre, post) = scan_fmt!(l,
                                    "Step {} must be finished before step {} can begin.",
                                    char, char);
        graph.add_edge(pre.unwrap(), post.unwrap());
    }

    //println!("graph: {:#?}", graph);
    graph.topo_sort()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(ordering(&input), "CABDFE");
    }

}
