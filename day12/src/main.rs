extern crate day12;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let contents = util::string_from_file(&filename);

    let result1 = day12::plant_sum(&contents, 20);
    println!("Part 1 Result: {}", result1);

    // Sum @ 200: 4528
    // beyond this point the sum increments by 20 at each step
    let steps: isize = 50000000000;
    let result2: isize = 4528 + ((steps - 201) * 20);
    println!("Part 2 Result: {}", result2);
}
