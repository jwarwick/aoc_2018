extern crate util;

use std::collections::HashMap;

fn main() {
    let depth = 10_647;
    let target =  (7, 770);

    let result1 = risk_level(&depth, &target);
    println!("Part 1 Result: {}", result1);
}

fn risk_level(depth: &usize, target: &(usize, usize)) -> usize {
    let cave = Cave::new(depth, target);
    cave.print();
    cave.risk_level()
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
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Loc {
    x: usize,
    y: usize,
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
    depth: usize,
}

impl Cave {
    fn new(depth: &usize, (target_x, target_y): &(usize, usize)) -> Cave {
        let target = Loc {x: *target_x, y: *target_y};
        let mut geologic: Geologic = HashMap::new();
        let mut erosion: Erosion = HashMap::new();

        Cave::erosion_level(&target, depth, &mut geologic, &mut erosion);
        geologic.insert(target, 0);
        geologic.insert(Loc{x: 0, y: 0}, 0);
        let result = (0 + depth) % 20183;
        erosion.insert(target, result);
        erosion.insert(Loc{x: 0, y: 0}, result);

        let mut kind: Kind = HashMap::new();
        for (l, e) in erosion.iter() {
            let k = Type::from_erosion_level(e);
            kind.insert(*l, k);
        }

        Cave {geologic, erosion, kind, target, depth: *depth}
    }

    fn erosion_level(l: &Loc, depth: &usize, geologic: &mut Geologic, erosion: &mut Erosion) -> usize {
        if let Some(v) = erosion.get(l) {
            return *v;
        }

        let g = Cave::geologic_index(&l, &depth, geologic, erosion);
        let result = (g + depth) % 20183;
        erosion.insert(*l, result);
        result
    }

    fn geologic_index(l: &Loc, depth: &usize, geologic: &mut Geologic, erosion: &mut Erosion) -> usize {
        if let Some(v) = geologic.get(l) {
            return *v;
        }

        let result = 
            {
                if l.x == 0 && l.y == 0 {
                    0
                } else if l.y == 0 {
                    l.x * 16807
                } else if l.x == 0 {
                    l.y * 48271
                } else {
                    Cave::erosion_level(&Loc {x: l.x-1, y: l.y}, &depth,  geologic,  erosion) *
                        Cave::erosion_level(&Loc {x: l.x, y: l.y-1}, &depth,  geologic,  erosion)
                }};
        geologic.insert(*l, result);
        result
    }

    fn risk_level(&self) -> usize {
        let mut total = 0;
        for (_l, k) in self.kind.iter() {
            total = total + k.risk();
        }
        total
    }

    fn print(&self) {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
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

}
