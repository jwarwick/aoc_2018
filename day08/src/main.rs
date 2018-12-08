extern crate day08;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result1 = day08::metadata_sum(&contents);
    println!("Part 1 Result: {}", result1);

    //let result2 = day07::parallel_ordering(&contents, 5, 60);
    //println!("Part 2 Result: {}", result2);
}
