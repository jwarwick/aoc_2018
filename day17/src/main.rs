extern crate util;
#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;

fn main() {
    let filename = util::get_argument("input.txt");
    let contents = util::string_from_file(&filename);

    let result1 = gravity_fill(&contents);
    println!("Part 1 Result: {}", result1);
}

fn gravity_fill(contents: &str) -> usize {
    let mut scan = Scan::new(&contents);
    println!("{}, {}", scan.min_y, scan.max_y);
    scan.fill();
    //scan.print();
    scan.reachable()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Type {
    Source,
    Clay,
    DrySand,
    StandingWater,
    WetSand,
    Invalid,
}

impl Type {
    fn is_wet(&self) -> bool {
        *self == Type::WetSand || *self == Type::StandingWater
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
        self.squares.insert(start, Type::WetSand);
        let mut to_visit: Vec<Loc> = vec![start];

        while !to_visit.is_empty() {
            let curr = to_visit.pop().expect("A value on the visit list");
            let (down, down_type) = self.down(&curr);

            if down_type == Type::DrySand {
                self.squares.insert(down, Type::WetSand);
                to_visit.push(curr);
                to_visit.push(down);
            } else if down_type == Type::Clay || down_type == Type::StandingWater {
                let (can_fill_left, left_end) = self.check_left(&curr);
                let (can_fill_right, right_end) = self.check_right(&curr);
                if can_fill_left && can_fill_right {
                    self.make_row_wet(&curr);
                } else {
                    if None != right_end {
                        let drip = right_end.expect("Right drip");
                        to_visit.push(drip);
                    }
                    if None != left_end {
                        let drip = left_end.expect("Left drip");
                        to_visit.push(drip);
                    }
                }
            }
        }
    }

    fn check_left(&mut self, curr: &Loc) -> (bool, Option<Loc>) {
        self.check_end(&curr, -1)
    }

    fn check_right(&mut self, curr: &Loc) -> (bool, Option<Loc>) {
        self.check_end(&curr, 1)
    }

    fn check_end(&mut self, curr: &Loc, step: isize) -> (bool, Option<Loc>) {
        let new_loc = Loc{x: curr.x+step, y: curr.y};
        let (new_loc, kind) = self.get_loc(&new_loc);
        if kind == Type::Clay {
            return (true, None);
        }
        self.squares.insert(new_loc, Type::WetSand);
        let (_new_down, down_kind) = self.down(&new_loc);
        if down_kind == Type::DrySand {
            return (false, Some(new_loc));
        } else if down_kind == Type::WetSand {
            return (false, None);
        }
        self.check_end(&new_loc, step)
    }

    fn make_row_wet(&mut self, loc: &Loc) {
        self.squares.insert(*loc, Type::StandingWater);
        self.make_left_wet(&loc);
        self.make_right_wet(&loc);
    }

    fn make_left_wet(&mut self, loc: &Loc) {
        let (left_loc, kind) = self.left(loc);
        match kind {
            Type::WetSand => {
                self.squares.insert(left_loc, Type::StandingWater);
                self.make_left_wet(&left_loc);
            },
            _ => (),
        }
    }

    fn make_right_wet(&mut self, loc: &Loc) {
        let (right_loc, kind) = self.right(loc);
        match kind {
            Type::WetSand => {
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
                        Some(Type::WetSand) => '|',
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
    fn parse_test() {
        let contents = util::string_from_file(TEST_FILE);
        let scan = Scan::new(&contents);
        assert_eq!(scan.min_y, 1);
        assert_eq!(scan.max_y, 13);
        assert_eq!(scan.x_range(), (495, 506));
    }

}
