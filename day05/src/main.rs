extern crate day05;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result1 = day05::react_count(&contents);
    println!("Part 1 Result: {}", result1);

    let result2 = day05::best_polymer_count(&contents);
    println!("Part 2 Result: {}", result2);
}
