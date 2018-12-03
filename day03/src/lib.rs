extern crate util;
#[macro_use] extern crate itertools;
#[macro_use] extern crate scan_fmt;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Rect {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn points(&self) -> Vec<(i32, i32)> {
        let end_x = self.x + self.w;
        let end_y = self.y + self.h;
        iproduct!(self.x..end_x, self.y..end_y).collect()
    }
}

pub fn parse_file(filename: &str) -> Vec<Rect> {
    let contents = util::string_from_file(filename);
    parse_string(&contents)
}

fn parse_string(string: &str) -> Vec<Rect> {
    string.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Rect {
    let (idw, xw, yw, ww, hw) = scan_fmt!(line,
                                          "#{d} @ {d},{d}: {d}x{d}",
                                          i32, i32, i32, i32, i32);
    let id = idw.unwrap();
    let x = xw.unwrap();
    let y = yw.unwrap();
    let w = ww.unwrap();
    let h = hw.unwrap();

    Rect {id, x, y, w, h}
}

pub fn overlapped_area(input: &Vec<Rect>) -> usize {
    let mut all_points = HashMap::new();
    for r in input {
        let points = r.points();
        for p in points {
            let counter = all_points.entry(p).or_insert(0);
            *counter += 1;
        }
    }
    //println!("All Points: {:?}", all_points);
    //let some_points: Vec<((i32, i32), i64)> = all_points
    all_points
        .into_iter()
        .filter(|&(_, v)| v != 1)
        .count()
    //    .collect();
    //println!("Some Points: {:?}", some_points);
    //some_points.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input = "#123 @ 3,2: 5x4";
        let r = Rect{id: 123, x: 3, y: 2, w: 5, h: 4};
        assert_eq!(parse_line(&input), r);
    }

    #[test]
    fn parse_lines() {
        let input = "#123 @ 3,2: 5x4\n#73 @ 300,2: 50x47";
        let r1 = Rect{id: 123, x: 3, y: 2, w: 5, h: 4};
        let r2 = Rect{id: 73, x: 300, y: 2, w: 50, h: 47};
        assert_eq!(parse_string(&input), vec![r1, r2]);
    }

    #[test]
    fn rect_points() {
        let r = Rect{id: 123, x: 3, y: 2, w: 2, h: 3};
        let points = vec![
            (3, 2), (3, 3), (3, 4),
            (4, 2), (4, 3), (4, 4),];
        assert_eq!(r.points(), points);
    }

    #[test]
    fn overlap() {
        let input ="#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n #3 @ 5,5: 2x2";
        let rects = parse_string(&input);
        assert_eq!(overlapped_area(&rects), 4);
    }
}
