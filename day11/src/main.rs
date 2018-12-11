extern crate day11;
extern crate util;

fn main() {
    let key = 7400;
    let ((x, y, _size), _power) = day11::largest_total_power(key, 3);
    println!("Part 1 Result: {},{}", x, y);

    let ((x, y, size), _power) = day11::largest_total_power_any(key);
    println!("Part 2 Result: {},{},{}", x, y, size);
}
