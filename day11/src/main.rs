extern crate day11;
extern crate util;

fn main() {
    let key = 7400;
    let ((x, y), _power) = day11::largest_total_power(key);
    println!("Part 1 Result: {},{}", x, y);

    //let result2 = day08::compute_root_value(&contents);
    //println!("Part 2 Result: {}", result2);
}
