extern crate day06;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result1 = day06::largest_area(&contents);
    println!("Part 1 Result: {}", result1);

    let result2 = day06::safe_region(&contents, 10000);
    println!("Part 2 Result: {}", result2);
}
