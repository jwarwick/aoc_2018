extern crate util;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let contents = util::string_from_file("input.txt");

    let (result1, result2) = shortest_path(&contents);
    println!("Part 1 Result: {}", result1);
    println!("Part 2 Result: {}", result2);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Loc {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq)]
enum SquareType {
    Start,
    Room,
    Wall,
    Door,
    Unknown,
}

impl SquareType {
    fn char(&self) -> char {
        match self {
            SquareType::Start => 'X',
            SquareType::Room => '.',
            SquareType::Wall => '#',
            SquareType::Door => 'D',
            SquareType::Unknown => '?',
        }
    }
}

#[derive(Debug)]
struct Map {
    squares: HashMap<Loc, SquareType>,
}


impl Map {
    fn build(content: &str) -> Map {
        let mut map = Map {squares: HashMap::new()};
        let mut curr = Loc {x: 0, y: 0};
        let mut branches: Vec<Loc> = Vec::new();
        map.add_room(&curr, SquareType::Start);

        for c in content.chars() {
            match c {
                '^' => continue,
                '$' => continue,
                'W' => {
                    map.add_door(&Loc {x: curr.x - 1, y: curr.y}, &Loc {x: curr.x - 2, y: curr.y});
                    curr.x = curr.x - 2;
                }
                'E' => {
                    map.add_door(&Loc {x: curr.x + 1, y: curr.y}, &Loc {x: curr.x + 2, y: curr.y});
                    curr.x = curr.x + 2;
                }
                'N' => {
                    map.add_door(&Loc {x: curr.x, y: curr.y - 1}, &Loc {x: curr.x, y: curr.y - 2});
                    curr.y = curr.y - 2;
                }
                'S' => {
                    map.add_door(&Loc {x: curr.x, y: curr.y + 1}, &Loc {x: curr.x, y: curr.y + 2});
                    curr.y = curr.y + 2;
                }
                '(' => {
                    branches.push(curr);
                },
                ')' => {
                    curr = branches.pop().expect("Stop branch location");
                },
                '|' => {
                    curr = branches.pop().expect("Pipe branch location");
                    branches.push(curr);
                },
                _ => continue,
            }
        }

        map.add_walls();
        map
    }

    fn furthest_room(&mut self, start: &Loc) -> usize {
        let mut flat = self.room_distances(start);
        flat.sort();
        flat.reverse();
        *flat.first().expect("A furthest node")
    }

    fn min_distance(&mut self, start: &Loc, min: usize) -> usize {
        let mut flat = self.room_distances(start);
        flat = flat.iter().cloned().filter(|x| *x >= min).collect();
        flat.sort();
        flat.reverse();
        flat.len()
    }

    fn room_distances(&mut self, start: &Loc) -> Vec<usize> {
        let mut distances: HashMap<Loc, usize> = HashMap::new();
        distances.insert(*start, 0);
        let mut to_visit: VecDeque<(Loc, usize)> = VecDeque::new();
        to_visit.push_back((*start, 0));

        while !to_visit.is_empty() {
            let (curr, dist) = to_visit.pop_front().expect("Node to visit");
            let new_dist = dist + 1;
            let neighbors = self.neighbors(&curr);
            for n in neighbors {
                let e = distances.entry(n).or_insert(std::usize::MAX);
                if new_dist < *e {
                    *e = new_dist;
                    to_visit.push_back((n, new_dist));
                }
            }
        }
        
        let flat: Vec<_> = distances.iter().map(|(_l, d)| d).cloned().collect();
        flat
    }

    fn neighbors(&mut self, curr: &Loc) -> Vec<Loc> {
        let mut n: Vec<Loc> = Vec::new();
        for (x, y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let c = self.get(curr.x + x, curr.y + y);
            if c == SquareType::Door {
                n.push(Loc {x: curr.x + (x * 2), y: curr.y + (y * 2)});
            }
        }
        n
    }

    fn print(&mut self) {
        let ((min_x, max_x), (min_y, max_y)) = self.range();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = self.get(x, y);
                print!("{}", c.char());
            }
            print!("\n");
        }
    }

    fn get(&mut self, x: isize, y: isize) -> SquareType {
        self.squares.entry(Loc{x, y}).or_insert(SquareType::Unknown).clone()
    }

    fn add_walls(&mut self) {
        let clones = self.squares.clone();
        let unknowns: Vec<_> = clones.iter().filter(|(_l, v)| **v == SquareType::Unknown).collect();
        for (l, _v) in unknowns {
            let e = self.squares.entry(*l).or_insert(SquareType::Unknown);
            *e = SquareType::Wall;
        }
    }

    fn add_door(&mut self, door: &Loc, room: &Loc) {
        self.squares.insert(*door, SquareType::Door);
        self.add_room(room, SquareType::Room);
    }

    fn add_room(&mut self, l: &Loc, t: SquareType) {
        self.squares.insert(*l, t);
        self.maybe_add(Loc {x: l.x - 1, y: l.y - 1}, SquareType::Wall);
        self.maybe_add(Loc {x: l.x + 1, y: l.y - 1}, SquareType::Wall);
        self.maybe_add(Loc {x: l.x - 1, y: l.y + 1}, SquareType::Wall);
        self.maybe_add(Loc {x: l.x + 1, y: l.y + 1}, SquareType::Wall);
        self.maybe_add(Loc {x: l.x, y: l.y - 1}, SquareType::Unknown);
        self.maybe_add(Loc {x: l.x + 1, y: l.y}, SquareType::Unknown);
        self.maybe_add(Loc {x: l.x, y: l.y + 1}, SquareType::Unknown);
        self.maybe_add(Loc {x: l.x - 1, y: l.y}, SquareType::Unknown);
    }

    fn maybe_add(&mut self, l: Loc, t: SquareType) {
        let curr = self.squares.entry(l).or_insert(SquareType::Unknown);

        if *curr == SquareType::Unknown {
            *curr = t;
        }
    }

    fn range(&self) -> ((isize, isize), (isize, isize)) {
        let mut min_x = 10000;
        let mut max_x = -10000;
        let mut min_y = 10000;
        let mut max_y = -10000;

        for (l, _v) in &self.squares {
            if l.x < min_x {
                min_x = l.x;
            }
            if l.x > max_x {
                max_x = l.x;
            }

            if l.y < min_y {
                min_y = l.y;
            }
            if l.y > max_y {
                max_y = l.y;
            }
        }

        ((min_x, max_x), (min_y, max_y))
    }
}

fn shortest_path(contents: &str) -> (usize, usize) {
    let mut map = Map::build(contents);
    map.print();
    let start = Loc{x: 0, y: 0};
    let result1 = map.furthest_room(&start);
    let result2 = map.min_distance(&start, 1000);
    (result1, result2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let contents = "^WNE$";
        assert_eq!(shortest_path(&contents), (3, 0));
    }

    #[test]
    fn alternates_paths() {
        let contents = "^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(shortest_path(&contents), (10, 0));
    }

    #[test]
    fn empty_options() {
        let contents = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(shortest_path(&contents), (18, 0));
    }

    #[test]
    fn longer() {
        let contents = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(shortest_path(&contents), (23, 0));
    }

    #[test]
    fn longest() {
        let contents = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(shortest_path(&contents), (31, 0));
    }
}
