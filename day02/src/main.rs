extern crate day02;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let input = day02::parse_file(&filename);

    let result1 = day02::checksum(&input);
    println!("Part 1 Result: {}", result1);

    let result2 = day02::common_letters(&input);
    println!("Part 2 Result: {}", result2);
}
