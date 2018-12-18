extern crate util;
#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;

fn main() {
    let contents = util::string_from_file("input.txt");

    let result1 = gravity_fill(&contents);
    println!("Part 1 Result: {}", result1);
}

fn gravity_fill(contents: &str) -> usize {
    let mut scan = Scan::new(&contents);
    println!("{}, {}", scan.min_y, scan.max_y);
    scan.fill();
    scan.print();
    scan.reachable()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Type {
    Source,
    Clay,
    DrySand,
    StandingWater,
    WetSand(isize, isize),
    Invalid,
}

impl Type {
    fn is_wet(&self) -> bool {
        match self {
            Type::WetSand(_x, _y) => true,
            Type::StandingWater => true,
            _ => false
        }
    }

    fn wet_or_wall(&self) -> bool {
        match self {
            Type::WetSand(_x, _y) => true,
            Type::Clay => true,
            _ => false
        }
    }

    fn dry_or_wall(&self) -> bool {
        *self == Type::DrySand || *self == Type::Clay
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Loc {
    x: isize,
    y: isize
}

#[derive(Debug, Clone)]
struct Scan {
    squares: HashMap<Loc, Type>,
    min_y: isize,
    max_y: isize,
}

impl Scan {
    fn new(contents: &str) -> Scan {
        let mut scan = Scan {squares: HashMap::new(), min_y: 0, max_y: 0};
        for l in contents.lines() {
            scan.add_clay(l);
        }

        {
            let keys = scan.squares.keys();
            let ys: Vec<_> = keys.map(|l| l.y).collect();
            let (min, max) = Scan::range(&ys);
            scan.min_y = min;
            scan.max_y = max;
        }

        scan.squares.insert(Loc{x: 500, y:0}, Type::Source);
        scan
    }

    fn fill(&mut self) {
        let start = Loc{x: 500, y:0};
        self.squares.insert(start, Type::WetSand(start.x, start.y));
        let mut to_visit: Vec<(Loc, Option<Loc>)> = vec![(start, Some(start))];

        let mut cnt = 0;
        while !to_visit.is_empty() { // && cnt < 1000 {
            //println!("\n\n\nList: {:?}", to_visit);
            cnt += 1;
            let (curr, parent) = to_visit.pop().expect("A value on the visit list");
            let (_curr_loc, curr_type) = self.get_loc(&curr);
            if curr_type == Type::StandingWater {
                continue;
            }
            let (down, down_type) = self.down(&curr);
            //println!("\n\nCurr: {:?}", curr);
            //println!("Down type: {:?}", down_type);
            //self.print();

            if down_type == Type::DrySand {
                to_visit.push((curr, parent));
                to_visit.push((down, Some(curr)));
                self.squares.insert(down, Type::WetSand(curr.x, curr.y));

            } else if down_type == Type::WetSand(curr.x, curr.y) {
                let (_left, down_left_type) = self.left(&down);
                let (_right, down_right_type) = self.right(&down);

                if Type::wet_or_wall(&down_right_type) && Type::wet_or_wall(&down_left_type) {
                    //println!("Making wet");
                    self.squares.insert(down, Type::StandingWater);
                    self.make_left_wet(&down);
                    self.make_right_wet(&down);
                    to_visit.push((curr, parent));
                }

            } else if down_type == Type::Invalid {
                //println!("Filtering...");
                //println!("Parent = {:?}", parent);
                let mut to_remove = vec![parent];
                while !to_remove.is_empty() {
                    //println!("Remove list: {:?}", to_remove);
                    let next_remove = to_remove.pop().expect("Removable node to pop");
                    //println!("Removing node {:?}", next_remove);
                    if None != next_remove {
                        let parent_branch = next_remove.expect("Parent branch");
                        let nodes_remove: Vec<_> = to_visit.iter().cloned().filter(|(l, _b)| *l == parent_branch).collect();
                        let mut parent_removes: Vec<_> = nodes_remove.iter().cloned().map(|(_l, p)| p).collect();
                        to_remove.append(&mut parent_removes);

                        to_visit = to_visit.iter().cloned().filter(|(l, _b)| *l != parent_branch).collect();
                    }
                }

            } else if down_type == Type::Clay || down_type == Type::StandingWater {
                //println!("Hit clay or water");

                let (left, left_type) = self.left(&curr);
                let (right, right_type) = self.right(&curr);
                let parent_val = parent.unwrap();

                if left_type == Type::DrySand {
                    //println!("Pushing left");
                    to_visit.push((left, parent));
                    self.squares.insert(left, Type::WetSand(parent_val.x, parent_val.y));
                }

                if right_type == Type::DrySand {
                    //println!("Pushing right");
                    to_visit.push((right, parent));
                    self.squares.insert(right, Type::WetSand(parent_val.x, parent_val.y));
                }
            }
        }
    }

    fn make_left_wet(&mut self, loc: &Loc) {
        let (left_loc, kind) = self.left(loc);
        match kind {
            Type::WetSand(_x, _y) => {
                self.squares.insert(left_loc, Type::StandingWater);
                self.make_left_wet(&left_loc);
            },
            _ => (),
        }
    }

    fn make_right_wet(&mut self, loc: &Loc) {
        let (right_loc, kind) = self.right(loc);
        match kind {
            Type::WetSand(_x, _y) => {
                self.squares.insert(right_loc, Type::StandingWater);
                self.make_right_wet(&right_loc);
            },
            _ => (),
        }
    }

    fn left(&self, loc: &Loc) -> (Loc, Type) {
        let new_loc = Loc{x: loc.x-1, y: loc.y};
        self.get_loc(&new_loc)
    }

    fn right(&self, loc: &Loc) -> (Loc, Type) {
        let new_loc = Loc{x: loc.x+1, y: loc.y};
        self.get_loc(&new_loc)
    }

    fn down(&self, loc: &Loc) -> (Loc, Type) {
        let new_loc = Loc{x: loc.x, y: loc.y+1};
        self.get_loc(&new_loc)
    }

    fn up(&self, loc: &Loc) -> (Loc, Type) {
        let new_loc = Loc{x: loc.x, y: loc.y-1};
        self.get_loc(&new_loc)
    }

    fn get_loc(&self, loc: &Loc) -> (Loc, Type) {
        if loc.y > self.max_y {
            return (*loc, Type::Invalid);
        }

        let d = self.squares.get(&loc);
        match d {
            None => (*loc, Type::DrySand),
            Some(t) => (*loc, *t),
        }
    }

    //fn reachable(&self) -> usize {
    //    let vals: Vec<Type> = self.squares.values().cloned().collect();
    //    let filtered: Vec<Type> = vals.iter().filter(|t| t.is_wet()).cloned().collect();
    //    filtered.len()
    //}

    fn reachable(&self) -> usize {
        let vals: Vec<_> = self.squares.iter().filter(|(_l, t)| t.is_wet()).collect();
        let filtered: Vec<_> = vals.iter().filter(|(l, _t)| l.y <= self.max_y && l.y >= self.min_y).collect();
        filtered.len()
    }

    fn x_range(&self) -> (isize, isize) {
        let keys = self.squares.keys();
        let xs: Vec<_> = keys.map(|l| l.x).collect();
        Scan::range(&xs)
    }

    fn range(vals: &Vec<isize>) -> (isize, isize) {
        let max = vals.iter().max().expect("A max value");
        let min = vals.iter().min().expect("A min value");
        (*min, *max)
    }

    fn add_clay(&mut self, line: &str) {
        let (arg1w, valw, _arg2, startw, endw) = scan_fmt!(line,
                                                           "{}={d}, {}={d}..{d}",
                                                           char, isize, char, isize, isize);
        let (val, start, end) =
            (valw.expect("val 1"), startw.expect("Start val"), endw.expect("End val"));


        match arg1w.expect("Leading var") {
            'x' => self.add_clay_range(val, val, start, end),
            'y' => self.add_clay_range(start, end, val, val),
            c => println!("Unexpected leading character: {}", c),
        };
    }

    fn add_clay_range(&mut self, x_min: isize, x_max: isize, y_min: isize, y_max: isize) {
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                self.squares.insert(Loc{x, y}, Type::Clay);
            }
        }
    }

    fn print(&self) {
        let (min_x, max_x) = self.x_range().clone();
        println!("({}, {})", min_x - 1, self.min_y - 1);
        for y in self.min_y-1 ..= self.max_y+1 {
            for x in min_x-1 ..= max_x+1 {
                let c =
                    match self.squares.get(&Loc{x,y}) {
                        Some(Type::Source) => '+',
                        Some(Type::DrySand) => '.',
                        Some(Type::WetSand(_x, _y)) => '|',
                        Some(Type::Clay) => '#',
                        Some(Type::StandingWater) => '~',
                        _ => '.'
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

    static TEST_FILE: &str = "test_input.txt";

    #[test]
    fn test_sample1() {
        let contents = util::string_from_file(TEST_FILE);
        assert_eq!(gravity_fill(&contents), 57);
    }

    #[test]
    fn test_sample2() {
        let contents = util::string_from_file("test_input2.txt");
        assert_eq!(gravity_fill(&contents), 53);
    }

    #[test]
    fn test_sample3() {
        let contents = util::string_from_file("test_input3.txt");
        assert_eq!(gravity_fill(&contents), 53);
    }

    #[test]
    fn parse_test() {
        let contents = util::string_from_file(TEST_FILE);
        let scan = Scan::new(&contents);
        assert_eq!(scan.min_y, 1);
        assert_eq!(scan.max_y, 13);
        assert_eq!(scan.x_range(), (495, 506));
    }

}
