extern crate day13;
extern crate util;
use std::fs;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let s = fs::read_to_string(&filename)
        .expect("Something went wrong reading the input file");

    let (x, y) = day13::first_crash(&s);
    println!("Part 1 Result: {},{}", x, y);
}
