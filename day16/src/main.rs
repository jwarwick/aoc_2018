extern crate day16;
extern crate util;

fn main() {
    let filename1 = util::get_argument("input_part1.txt");
    println!("Using file: {}", filename1);

    let contents1 = util::string_from_file(&filename1);

    let result1 = day16::count_multiples(&contents1);
    println!("Part 1 Result: {}", result1);
}
