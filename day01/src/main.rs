extern crate day01;
extern crate util;

fn main() {
    let filename = util::get_argument("input.txt");
    println!("Using file: {}", filename);

    let result = day01::calibrate_from_file(&filename);
    println!("Part 1 Result: {}", result);

    //let result = day01::rotated_captcha_from_file(&filename);
    //println!("Part 2 Result: {}", result);
}
