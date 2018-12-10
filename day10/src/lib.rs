extern crate util;
#[macro_use] extern crate scan_fmt;
extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Sky {
    lights: Vec<Light>,
}

impl Sky {
    fn new(contents: &str) -> Sky {
        let mut lights: Vec<Light> = Vec::new();
        for l in contents.lines() {
            let (x, y, delta_x, delta_y) = scan_fmt!(l,
                                                     "position=<{},  {}> velocity=<{}, {}>",
                                                     isize, isize, isize, isize);
            lights.push(
                Light {x: x.unwrap(), y: y.unwrap(), delta_x: delta_x.unwrap(), delta_y: delta_y.unwrap()});
        }
        Sky {lights}
    }

    fn step(&mut self) -> usize {
        self.lights = self.lights.iter().
            cloned().
            map(|mut l| {
                l.x += l.delta_x;
                l.y += l.delta_y;
                l}).
            collect();

        let ((minx, maxx), (miny, maxy)) = self.bounding_box();
        let width = maxx - minx;
        let height = maxy - miny;
        (height * width) as usize
    }

    fn bounding_box(&self) -> ((isize, isize), (isize, isize)) {
        let (minxl, maxxl) = self.lights.iter().minmax_by(|a, b| a.x.cmp(&b.x)).into_option().expect("X range");
        let (minyl, maxyl) = self.lights.iter().minmax_by(|a, b| a.y.cmp(&b.y)).into_option().expect("Y range");
        let minx = minxl.x;
        let maxx = maxxl.x;
        let miny = minyl.y;
        let maxy = maxyl.y;
        ((minx, maxx), (miny, maxy))
    }

    fn print(&self) {
        let ((minx, maxx), (miny, maxy)) = self.bounding_box();
        let points: HashSet<(isize, isize)> = self.lights.iter().map(|l| (l.x, l.y)).collect();

        for y in miny..=maxy {
            for x in minx..=maxx {
                if points.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Light {
    x: isize,
    y: isize,
    delta_x: isize,
    delta_y: isize,
}

pub fn simulate(contents: &str, steps: &usize) -> usize {
    let mut lights = Sky::new(contents);
    let mut area: Vec<(usize, usize)> = Vec::with_capacity(*steps);

    for idx in 0..*steps {
        let bb = lights.step();
        area.push((bb, idx));
    }

    lights.print();
    //println!("Areas: {:?}", area);
    let min = area.iter().min().unwrap();
    println!("Minimum: {:?}", min);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let filename = util::get_argument("test_input.txt");
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents, &3), 7);
    }
}
