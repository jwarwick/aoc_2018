extern crate day03;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let input = day03::parse_file(&filename);

    let (area, idx) = day03::overlapped_area(&input);
    println!("Part 1 Result: {}", area);
    println!("Part 2 Result: {}", idx);
}
