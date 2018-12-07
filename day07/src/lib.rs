extern crate util;
#[macro_use] extern crate scan_fmt;

use std::collections::HashSet;
use std::collections::HashMap;

type NodeType = HashMap<char, (bool, HashSet<char>)>;

#[derive(Debug, Clone)]
struct Graph {
    nodes: NodeType,
}

impl Graph {
    fn new() -> Graph {
        Graph {nodes: HashMap::new()}
    }

    fn add_edge(&mut self, from: char, to: char) {
        self.nodes.entry(from).or_insert((false, HashSet::new()));
        let (_c, to_node) = self.nodes.entry(to).or_insert((false, HashSet::new()));
        to_node.insert(from);
    }

    fn topo_sort(&self) -> String {
        let mut n = self.nodes.clone();
        let mut output: String = String::new();

        while !n.is_empty() {
            let s = Graph::next_node(&mut n).expect("Didn't find an edge node");
            n.remove(&s);
            Graph::remove_edges_from(s, &mut n);
            output.push(s);
        }

        output
    }

    fn remove_edges_from(c: char, nodes: &mut NodeType) {
        for (_claim, val) in nodes.values_mut() {
            val.remove(&c);
        }
    }

    fn claim(c: char, nodes: &mut NodeType) {
        if let Some((claim, _val)) = nodes.get_mut(&c) {
            *claim = true;
        }
    }

    fn next_node(nodes: &mut NodeType) -> Option<char> {
        let mut edges: Vec<_> = nodes.iter().filter(|(_k, (claim, v))| v.is_empty() && !claim).collect();
        edges.sort_by_key(|(&k, _v)| k);
        match edges.first() {
            None => None,
            Some((k, _v)) => Some(**k),
        }
    }

    fn parallel_topo_sort_time(&self, worker_cnt: usize, step_len: usize) -> usize {
        let mut n = self.nodes.clone();
        let mut workers: Vec<(char, usize)> = Vec::new();
        let mut t = 0;

        while !n.is_empty() || !workers.is_empty() {
            let fold_workers: Vec<(char, usize)> = Vec::new();
            workers = workers
                .iter()
                .fold(fold_workers, |mut acc, (c, t)|
                      if 0 == t-1 {
                          n.remove(&c);
                          Graph::remove_edges_from(*c, &mut n);
                          acc
                      } else {
                          acc.push((*c, t - 1));
                          acc
                      });

            while workers.iter().count() < worker_cnt {
                match Graph::next_node(&mut n) {
                    None => break,
                    Some(c) =>
                    {
                        workers.push((c, Graph::time(&c, &step_len)));
                        Graph::claim(c, &mut n);
                    },
                }
            }

            //println!("{}: {:?}", t, workers);
            t += 1;
        }
        t - 1
    }

    fn time(c: &char, step_len: &usize) -> usize {
        step_len + ((*c as usize) - ('A' as usize)) + 1
    }
}


pub fn ordering(s: &str) -> String {
    let graph = build_graph(s);
    graph.topo_sort()
}

pub fn parallel_ordering(s: &str, workers: usize, step_len: usize) -> usize {
    let graph = build_graph(s);
    graph.parallel_topo_sort_time(workers, step_len)
}

fn build_graph(s: &str) -> Graph {
    let mut graph = Graph::new();
    for l in s.lines() {
        let (pre, post) = scan_fmt!(l,
                                    "Step {} must be finished before step {} can begin.",
                                    char, char);
        graph.add_edge(pre.unwrap(), post.unwrap());
    }
    graph
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

    #[test]
    fn parallel_sample() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(parallel_ordering(&input, 2, 0), 15);
    }
}
