extern crate util;

use std::collections::HashMap;
use std::str::FromStr;

type Carts = Vec<Cart>;
type Map = HashMap<(usize, usize), Orientation>;
type Crash = Option<(usize, usize)>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
    Intersection,
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Cart {
    y: usize, // Y first for ordering purposes
    x: usize,
    heading: Heading,
    turn_idx: usize,
    turns: Vec<Turn>,
}

impl Cart {
    fn step(&self, map: &Map) -> (usize, usize, Heading, usize) {
        let curr_orient = map.get(&(self.x, self.y)).expect(&format!("Current map location {},{}", self.x, self.y));
        println!("{}, {} = {:?} going {:?}", self.x, self.y, curr_orient, self.heading);
        let mut new_x = self.x;
        let mut new_y = self.y;
        let mut new_heading = self.heading;
        let mut new_idx = self.turn_idx;
        match curr_orient {
            Orientation::Vertical =>
                match self.heading {
                    Heading::Up => new_y = self.y - 1,
                    Heading::Down => new_y = self.y + 1,
                    _ => (),
                },
            Orientation::Horizontal =>
                match self.heading {
                    Heading::Left => new_x = self.x - 1,
                    Heading::Right => new_x = self.x + 1,
                    _ => (),
                },
            Orientation::BackSlash =>
                match self.heading {
                    Heading::Left => {new_y = self.y - 1; new_heading = Heading::Up},
                    Heading::Right => {new_y = self.y + 1; new_heading = Heading::Down},
                    Heading::Up => {new_x = self.x - 1; new_heading = Heading::Left},
                    Heading::Down => {new_x = self.x + 1; new_heading = Heading::Right},
                },
            Orientation::ForwardSlash =>
                match self.heading {
                    Heading::Left => {new_y = self.y + 1; new_heading = Heading::Down},
                    Heading::Right => {new_y = self.y - 1; new_heading = Heading::Up},
                    Heading::Up => {new_x = self.x + 1; new_heading = Heading::Right},
                    Heading::Down => {new_x = self.x - 1; new_heading = Heading::Left},
                },
            Orientation::Intersection => 
            {
                let turn = self.turns[self.turn_idx];
                new_idx = (self.turn_idx + 1) % 3;
                match turn {
                    Turn::Straight => {
                        match self.heading {
                            Heading::Up => new_y = self.y - 1,
                            Heading::Down => new_y = self.y + 1,
                            Heading::Left => new_x = self.x - 1,
                            Heading::Right => new_x = self.x + 1,
                        };
                    },
                    Turn::Left => {
                        match self.heading {
                            Heading::Left => {new_y = self.y + 1; new_heading = Heading::Down},
                            Heading::Right => {new_y = self.y - 1; new_heading = Heading::Up},
                            Heading::Up => {new_x = self.x - 1; new_heading = Heading::Left},
                            Heading::Down => {new_x = self.x + 1; new_heading = Heading::Right},
                        };
                    },
                    Turn::Right => {
                        match self.heading {
                            Heading::Left => {new_y = self.y - 1; new_heading = Heading::Up},
                            Heading::Right => {new_y = self.y + 1; new_heading = Heading::Down},
                            Heading::Up => {new_x = self.x + 1; new_heading = Heading::Right},
                            Heading::Down => {new_x = self.x - 1; new_heading = Heading::Left},
                        };
                    },
                };
            },
        };

        (new_x, new_y, new_heading, new_idx)
    }
}

#[derive(Debug, Clone)]
struct State {
    carts: Carts,
    map: Map,
}

impl FromStr for State {
    type Err = ();
    fn from_str(s: &str) -> Result<State, ()> {
        let mut carts = Carts::new();
        let mut map = Map::new();

        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if 0 == y {
                    println!("{},{}: {}", x, y, c);
                }
                match State::parse_map(&c) {
                    Some(track) => map.insert((x, y), track),
                    None => None,
                };
                match State::parse_heading(&c) {
                    Some(heading) =>
                    {
                        match heading {
                            Heading::Up => map.insert((x, y), Orientation::Vertical),
                            Heading::Down => map.insert((x, y), Orientation::Vertical),
                            Heading::Left => map.insert((x, y), Orientation::Horizontal),
                            Heading::Right => map.insert((x, y), Orientation::Horizontal),
                        };
                        let turn_idx = 0;
                        let turns = vec![Turn::Left, Turn::Straight, Turn::Right];
                        carts.push(Cart {x, y, heading, turn_idx, turns})
                    },
                    None => (),
                };
            }
        }

        Ok(State {
            carts,
            map
        })
    }
}

impl State {
    fn parse_map(c: &char) -> Option<Orientation> {
        match c {
            '|' => Some(Orientation::Vertical),
            '-' => Some(Orientation::Horizontal),
            '+' => Some(Orientation::Intersection),
            '/' => Some(Orientation::ForwardSlash),
            '\\' => Some(Orientation::BackSlash),
            _ => None,
        }
    }

    fn parse_heading(c: &char) -> Option<Heading> {
        match c {
            '^' => Some(Heading::Up),
            'v' => Some(Heading::Down),
            '>' => Some(Heading::Right),
            '<' => Some(Heading::Left),
            _ => None,
        }
    }

    fn tick(&mut self) -> Crash {
        let mut crash: Crash = None;
        let mut new_carts: Carts = Vec::new();
        let cart_cnt = self.carts.len();
        self.carts.sort();
        self.carts.reverse();
        for _i in 0..cart_cnt {
            let cart = self.carts.pop().expect("Taking the first cart");
            println!("\tCart: {:?}", cart);
            let (x, y, heading, turn_idx) = cart.step(&self.map);
            if self.carts.iter().any(|c| c.x == x && c.y == y) ||
                new_carts.iter().any(|c| c.x == x && c.y == y) {
                    crash = Some((x, y));
                    println!("CRASH!!!!!");
                    break;
                }
            let turns = cart.turns.clone();
            new_carts.push(Cart{x, y, turn_idx, turns, heading});
        }
        self.carts = new_carts;
        crash
    }
}

pub fn first_crash(contents: &str) -> (usize, usize) {
    let mut state: State = contents.parse().expect("Read the file");
    let mut crash_x = 0;
    let mut crash_y = 0;

    for i in 1..1000 {
        println!("TICK {}", i);
        match state.tick() {
            Some((x, y)) =>
            {
                crash_x = x;
                crash_y = y;
                break;
            },
            None => (),
        };
    }
    
    (crash_x, crash_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn circle_track() {
        let contents =
            "/--<-\\
|    |
|    |
\\----/";
        println!("{}", contents);
        assert_eq!(first_crash(&contents), (0, 0));
    }
//
//    #[test]
//    fn intersections() {
//        let contents =
//            "/-----\\
//|     |
//|  /--+--\\
//|  |  |  |
//\\--+--/  |
//   |     |
//   \\-----/";
//        println!("{}", contents);
//        assert_eq!(first_crash(&contents), (7, 7));
//    }
//
    #[test]
    fn straight_track() {
        let contents =
            "|
v
|
|
|
^
|";
        println!("{}", contents);
        assert_eq!(first_crash(&contents), (0, 3));
    }

    #[test]
    fn long_sample() {
        let s = fs::read_to_string("test_input.txt")
            .expect("Something went wrong reading the input file");
        println!("{}", s);
        assert_eq!(first_crash(&s), (7, 3));
    }

    //#[test]
    //fn part1() {
    //    let s = fs::read_to_string("input.txt")
    //        .expect("Something went wrong reading the input file");
    //    println!("{}", s);
    //    assert_eq!(first_crash(&s), (7, 3));
    //}
}
