extern crate util;
extern crate itertools;
#[macro_use] extern crate scan_fmt;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;
use std::usize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Point, ()> {
        let (x, y) = scan_fmt!(s, "{}, {}", i32, i32);

        Ok(Point {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
}

impl Point {
    fn distance(&self, pt: &Point) -> i32 {
        (self.x - pt.x).abs() + (self.y - pt.y).abs()
    }

    fn total_distance(&self, pts: &Vec<Point>) -> i32 {
        pts.iter().map(|x| x.distance(self)).sum()
    }

    fn neighbors(&self) -> Vec<Point> {
        let mut n = Vec::new();
        n.push(Point {x: self.x, y: self.y - 1});
        n.push(Point {x: self.x, y: self.y + 1});
        n.push(Point {x: self.x - 1, y: self.y});
        n.push(Point {x: self.x + 1, y: self.y});
        n
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Location {
    pos: Point,
    parent: i32,
    dist: usize,
}

impl Location {
    fn neighbors(&self) -> Vec<Location> {
        let mut n = Vec::new();
        n.push(Location{pos: Point {x: self.pos.x, y: self.pos.y - 1}, parent: self.parent, dist: self.dist + 1});
        n.push(Location{pos: Point {x: self.pos.x, y: self.pos.y + 1}, parent: self.parent, dist: self.dist + 1});
        n.push(Location{pos: Point {x: self.pos.x - 1, y: self.pos.y}, parent: self.parent, dist: self.dist + 1});
        n.push(Location{pos: Point {x: self.pos.x + 1, y: self.pos.y}, parent: self.parent, dist: self.dist + 1});
        n
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    closest: i32,
    distance: usize,
}

pub fn safe_region(s: &str, cutoff: i32) -> i32 {
    let mut givens: Vec<Point> = Vec::new();
    for loc_str in s.lines() {
        let pt: Point = loc_str.parse().expect("Not a location string");
        givens.push(pt);
    }

    let center = get_center(&givens);

    let mut valid: HashSet<Point> = HashSet::new();
    let mut seen: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(center);

    while let Some(p) = queue.pop_front() {
        seen.insert(p);
        if p.total_distance(&givens) < cutoff {
            valid.insert(p);
            let neighbors = p.neighbors();
            for n in neighbors {
                if !seen.contains(&n) && !queue.contains(&n) {
                    queue.push_back(n);
                }
            }
        }
    }
    valid.iter().count() as i32
}

fn get_center(pts: &Vec<Point>) -> Point {
    let (x_min, x_max) = pts.iter().map(|p| p.x).minmax().into_option().unwrap();
    let (y_min, y_max) = pts.iter().map(|p| p.y).minmax().into_option().unwrap();
    Point{x: (x_max - x_min)/2, y: (y_max - y_min)/2}
}

pub fn largest_area(s: &str) -> i32 {
    let mut nodes: HashMap<Point, Node> = HashMap::new();
    let mut queue: VecDeque<Location> = VecDeque::new();
    for (idx, loc_str) in s.lines().enumerate() {
        let pt: Point = loc_str.parse().expect("Not a location string");
        let loc = Location {parent: idx as i32, dist: 0, pos: pt};
        queue.push_back(loc.clone());
        nodes.insert(pt, Node{closest: idx as i32, distance: 0});
    }

    let mut max = 500000;
    while let Some(p) = queue.pop_front() {
        let neighbors = p.neighbors();
        for n in neighbors {
            if update_node(&n, &mut nodes) && !queue.contains(&n) {
                queue.push_back(n);
            }
        }
        max -= 1;
        if max <= 0 {
            break;
        }
    }

    let mut active_ids: HashSet<i32> = HashSet::new();
    for i in queue {
        active_ids.insert(i.parent);
    }
    active_ids.insert(-1);

    println!("active: {:?}", active_ids);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for n in nodes.values() {
        if !active_ids.contains(&n.closest) {
            let counter = counts.entry(n.closest).or_insert(0);
            *counter += 1;
        }
    }
    println!("Counts: {:#?}", counts);

    *counts.values().max().expect("Didn't find a max value")
}

fn update_node(loc: &Location, nodes: &mut HashMap<Point, Node>) -> bool {
    let child_node = nodes.entry(loc.pos).or_insert(Node {closest: -2, distance: usize::MAX});
    if child_node.distance < loc.dist {
        false
    } else if child_node.distance == loc.dist && child_node.closest != loc.parent {
        (*child_node).closest = -1;
        false
    } else if child_node.distance > loc.dist {
        (*child_node).distance = loc.dist;
        (*child_node).closest = loc.parent;
        true
    } else {
        // reached node in same area with same distance
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        assert_eq!(largest_area(&input), 17);
    }

    #[test]
    fn safe() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        assert_eq!(safe_region(&input, 32), 16);
    }
}
