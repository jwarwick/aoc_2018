extern crate util;
extern crate z3;
#[macro_use] extern crate scan_fmt;

use std::str::FromStr;
//use std::collections::HashMap;
use z3::*;

fn main() {
    let filename = util::get_argument("input.txt");
    let content = util::string_from_file(&filename);

    let result1 = in_range(&content);
    println!("Part 1 Result: {}", result1);

    let result2 = dist_to_center(&content);
    println!("Part 2 Result: {}", result2);
}

fn in_range(content: &str) -> usize {
    let (largest, bots) = build_list(&content);
    let close: Vec<Bot> = bots.iter().cloned().filter(|x| largest.dist(x) <= largest.range).collect();
    close.len()
}

fn dist_to_center(content: &str) -> usize {
    let (_largest, bots) = build_list(&content);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let z = ctx.named_int_const("z");

    let optimizer = Optimize::new(&ctx);

    for b in bots {
        
    }

    let check = optimizer.check();
    if !check {
        println!("Failed to optimize the model");
        0
    } else {
        let model = optimizer.get_model();

        let xv = model.eval(&x).unwrap().as_i64().unwrap();
        let yv = model.eval(&y).unwrap().as_i64().unwrap();
        let zv = model.eval(&z).unwrap().as_i64().unwrap();
        println!("x: {}", xv);
        println!("y: {}", yv);
        println!("z: {}", zv);
        1
    }
}

fn build_list(content: &str) -> (Bot, Vec<Bot>) {
    let mut bots: Vec<Bot> = Vec::new();
    let mut largest = Bot {loc: Loc{x: 0, y: 0, z: 0}, range: std::usize::MIN};
    for l in content.lines() {
        let b: Bot = l.parse().expect("A bot");
        if b.range > largest.range {
            largest = b.clone();
        }
        bots.push(b);
    }
    (largest, bots)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Loc {
    x: isize,
    y: isize,
    z: isize,
}

impl Loc {
    fn dist(&self, b: &Loc) -> usize {
        let xd = self.x - b.x;
        let yd = self.y - b.y;
        let zd = self.z - b.z;
        (xd.abs() + yd.abs() + zd.abs()) as usize
    }
}

#[derive(Debug, Clone)]
struct Bot {
    loc: Loc,
    range: usize,
}

impl Bot {
    fn dist(&self, b: &Bot) -> usize {
        self.loc.dist(&b.loc)
    }

    fn intersect(&self, b: &Bot) -> bool {
        let d = self.dist(&b);
        d < self.range + b.range
    }
}

impl FromStr for Bot {
    type Err = ();
    fn from_str(s: &str) -> Result<Bot, ()> {
        let (x, y, z, r) = scan_fmt!(s,
                                     "pos=<{d},{d},{d}>, r={d}",
                                     isize, isize, isize, usize);

        Ok(Bot {
            loc:
                Loc {
                    x: x.expect("X value"),
                    y: y.expect("Y value"),
                    z: z.expect("Z value"),
                },
                range: r.expect("Range value"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radius_test() {
        let content = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

        assert_eq!(in_range(&content), 7);
    }

    #[test]
    fn teleport_test() {
        let content ="pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

        assert_eq!(dist_to_center(&content), 36);
    }
}
