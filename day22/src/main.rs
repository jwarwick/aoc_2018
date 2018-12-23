extern crate util;
extern crate petgraph;

use petgraph::Graph;
use petgraph::algo::astar;
use std::collections::HashMap;

fn main() {
    let depth = 10_647;
    let target =  (7, 770);

    let result1 = risk_level(&depth, &target);
    println!("Part 1 Result: {}", result1);

    let result2 = shortest_path(&depth, &target);
    println!("Part 2 Result: {}", result2);
}

fn risk_level(depth: &usize, (x, y): &(usize, usize)) -> usize {
    let cave = Cave::new(depth, &(*x, *y), &(x+5, y+5));
    //cave.print();
    cave.risk_level()
}

type Node = (Loc, Gear);
type Edge = usize;

fn shortest_path(depth: &usize, (x, y): &(usize, usize)) -> usize {
    let cave = Cave::new(depth, &(*x, *y), &(x+50, y+50));
    let mut graph = Graph::<Node, Edge>::new();

    let mut nodes: HashMap<(Loc, Gear), petgraph::prelude::NodeIndex> = HashMap::new();

    for (l, k) in cave.kind.iter() {
        let gear = k.valid_gear();
        let g1 = gear[0].clone();
        let g2 = gear[1].clone();
        let n1 = graph.add_node((*l, g1.clone()));
        let n2 = graph.add_node((*l, g2.clone()));
        graph.update_edge(n1, n2, 7);
        graph.update_edge(n2, n1, 7);
        nodes.insert((*l, g1), n1);
        nodes.insert((*l, g2), n2);
    }

    for ((l, g), n) in nodes.iter() {
        let neighbors = l.neighbors();
        for adj in neighbors {
            let adj_node = nodes.get(&(adj, g.clone()));
            if let Some(node) = adj_node {
                graph.update_edge(*n, *node, 1);
                graph.update_edge(*node, *n, 1);
            }
        }

    }

    let start_node = nodes.get(&(Loc{x: 0, y: 0}, Gear::Torch)).expect("start node");
    let target_node = nodes.get(&(Loc{x: *x, y: *y}, Gear::Torch)).expect("end node");

    let (dist, path) = astar(&graph, *start_node, |finish| finish == *target_node, |e| *e.weight(), |_| 0).expect("a path");

    //cave.print();
    dist
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Gear {
    Torch,
    Climbing,
    Neither,
}

#[derive(Debug, PartialEq)]
enum Type {
    Rocky,
    Narrow,
    Wet,
}

impl Type {
    fn risk(&self) -> usize {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow =>2,
        }
    }

    fn from_erosion_level(erosion_level: &usize) -> Type {
        let m = erosion_level % 3;
        match m {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => panic!("Erosion level out of range"),
        }
    }

    fn valid_gear(&self) -> Vec<Gear> {
        match self {
            Type::Rocky => vec![Gear::Torch, Gear::Climbing],
            Type::Wet => vec![Gear::Climbing, Gear::Neither],
            Type::Narrow => vec![Gear::Torch, Gear::Neither],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn neighbors(&self) -> Vec<Loc> {
        let mut v: Vec<Loc> = Vec::new();
        let px = self.x + 1;
        let py = self.y + 1;
        let mx = self.x as isize - 1;
        let my = self.y as isize - 1;

        v.push(Loc{x: self.x, y: py});
        v.push(Loc{x: px, y: self.y});
        if mx >= 0 {
            v.push(Loc{x: mx as usize, y: self.y});
        }
        if my >= 0 {
            v.push(Loc{x: self.x, y: my as usize});
        }

        v
    }
}

type Geologic = HashMap<Loc, usize>;
type Erosion = HashMap<Loc, usize>;
type Kind = HashMap<Loc, Type>;

#[derive(Debug)]
struct Cave {
    geologic: Geologic,
    erosion: Erosion,
    kind: Kind,
    target: Loc,
    corner: Loc,
    depth: usize,
}

impl Cave {
    fn new(depth: &usize, (target_x, target_y): &(usize, usize), (corner_x, corner_y): &(usize, usize)) -> Cave {
        let target = Loc {x: *target_x, y: *target_y};
        let corner = Loc {x: *corner_x, y: *corner_y};
        let mut geologic: Geologic = HashMap::new();
        let mut erosion: Erosion = HashMap::new();

        geologic.insert(Loc{x: 0, y: 0}, 0);
        let result = (0 + depth) % 20183;
        erosion.insert(Loc{x: 0, y: 0}, result);
        Cave::erosion_level(&corner, &target, depth, &mut geologic, &mut erosion);

        let mut kind: Kind = HashMap::new();
        for (l, e) in erosion.iter() {
            let k = Type::from_erosion_level(e);
            kind.insert(*l, k);
        }

        Cave {geologic, erosion, kind, corner, target, depth: *depth}
    }

    fn erosion_level(l: &Loc, target: &Loc, depth: &usize, geologic: &mut Geologic, erosion: &mut Erosion) -> usize {
        if let Some(v) = erosion.get(l) {
            return *v;
        }

        let g = Cave::geologic_index(&l, &target, &depth, geologic, erosion);
        let result = (g + depth) % 20183;
        erosion.insert(*l, result);
        result
    }

    fn geologic_index(l: &Loc, target: &Loc, depth: &usize, geologic: &mut Geologic, erosion: &mut Erosion) -> usize {
        if let Some(v) = geologic.get(l) {
            return *v;
        }

        let result = 
            {
                if l.x == 0 && l.y == 0 {
                    0
                } else if l.x == target.x && l.y == target.y {
                    0
                } else if l.y == 0 {
                    l.x * 16807
                } else if l.x == 0 {
                    l.y * 48271
                } else {
                    Cave::erosion_level(&Loc {x: l.x-1, y: l.y}, &target, &depth,  geologic,  erosion) *
                        Cave::erosion_level(&Loc {x: l.x, y: l.y-1}, &target, &depth,  geologic,  erosion)
                }};
        geologic.insert(*l, result);
        result
    }

    fn risk_level(&self) -> usize {
        let mut total = 0;
        for (l, k) in self.kind.iter() {
            if l.x <= self.target.x && l.y <= self.target.y {
                total = total + k.risk();
            }
        }
        total
    }

    fn print(&self) {
        for y in 0..=self.corner.y {
            for x in 0..=self.corner.x {
                let t = self.kind.get(&Loc{x, y}).expect("x,y value");
                let c = match t {
                    Type::Rocky => '.',
                    Type::Wet => '=',
                    Type::Narrow => '|',
                };
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erosion_type_test() {
        assert_eq!(Type::from_erosion_level(&510), Type::Rocky);
        assert_eq!(Type::from_erosion_level(&17317), Type::Wet);
        assert_eq!(Type::from_erosion_level(&8415), Type::Rocky);
        assert_eq!(Type::from_erosion_level(&1805), Type::Narrow);
    }
    
    #[test]
    fn sample_test() {
        let risk = risk_level(&510, &(10, 10));
        assert_eq!(risk, 114);
    }

    #[test]
    fn path_test() {
        let path = shortest_path(&510, &(10, 10));
        assert_eq!(path, 45);
    }

}
