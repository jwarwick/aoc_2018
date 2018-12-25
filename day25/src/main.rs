extern crate util;
#[macro_use] extern crate scan_fmt;
extern crate petgraph;

use petgraph::Graph;
use petgraph::prelude::*;
use petgraph::algo::connected_components;

fn main() {
    let filename = util::get_argument("input.txt");
    let content = util::string_from_file(&filename);

    let result1 = constellations(&content);
    println!("Part 1 Result: {}", result1);

    //let result2 = dist_to_center(&content);
    //println!("Part 2 Result: {}", result2);
}

fn constellations(content: &str) -> usize {
    let mut points: Vec<(Point, NodeIndex)> = Vec::new();
    let mut graph = Graph::<Point, isize>::new();

    for l in content.lines() {
        let (xw, yw, zw, tw) = scan_fmt!(l, "{d},{d},{d},{d}", isize, isize, isize, isize);
        let curr = Point {
            x: xw.expect("X value"),
            y: yw.expect("Y value"),
            z: zw.expect("Z value"),
            t: tw.expect("T value"),};

        let ni = graph.add_node(curr);
        for (p, pni) in points.iter() {
            let d = curr.distance(&p);
            if d <= 3 {
               graph.add_edge(ni, *pni, d);
               graph.add_edge(*pni, ni, d);
            }
        }
        points.push((curr, ni));
    }

    connected_components(&graph)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    t: isize,
}

impl Point {
    fn distance(&self, p: &Point) -> isize {
        (self.x - p.x).abs() +
            (self.y - p.y).abs() +
            (self.z - p.z).abs() +
            (self.t - p.t).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let content = "0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        assert_eq!(constellations(&content), 2);
    }

    #[test]
    fn ex2_test() {
        let content = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(constellations(&content), 4);
    }

    #[test]
    fn ex3_test() {
        let content = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(constellations(&content), 3);
    }

    #[test]
    fn ex4_test() {
        let content = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(constellations(&content), 8);
    }
}
