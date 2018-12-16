extern crate day15;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result1 = day15::simulate(&contents);
    println!("Part 1 Result: {}", result1);
}
