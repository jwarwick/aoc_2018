extern crate day14;
extern crate util;

fn main() {
    let input = 793061;
    let result1 = day14::next_ten(&input);
    println!("Part 1 Result: {}", result1);

    let result2 = day14::recipes_to(&input.to_string());
    println!("Part 2 Result: {}", result2);
}
