extern crate day16;
extern crate util;

fn main() {
    let contents1 = util::string_from_file("input_part1.txt");
    let contents2 = util::string_from_file("input_part2.txt");

    let (result1, result2) = day16::count_multiples(&contents1, &contents2);
    println!("Part 1 Result: {}", result1);
    println!("Part 2 Result: {}", result2);
}
