extern crate day04;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let input = day04::parse_file(&filename);

    let result1 = day04::strategy1(&input);
    println!("Part 1 Result: {}", result1);

    let result2 = day04::strategy2(&input);
    println!("Part 2 Result: {}", result2);
}
