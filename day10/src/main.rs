extern crate day10;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result1 = day10::simulate(&contents, &10595);
    println!("Part 1 Result: {}", result1);

    //let result2 = day08::compute_root_value(&contents);
    //println!("Part 2 Result: {}", result2);
}
