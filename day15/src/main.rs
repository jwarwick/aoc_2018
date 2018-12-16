extern crate day15;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result2 = day15::binary_search(&contents);
    println!("Part 2 Result: {}", result2);
}
