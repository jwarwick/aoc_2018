extern crate util;
extern crate device;

use device::Device;

fn main() {
    let contents = util::string_from_file("input.txt");

    let r0 = 10780777;
    let result1 = activation_system(&contents, [r0, 0, 0, 0, 0, 0]);
    println!("Part 1 Result: {}", result1);

    activation_endless(&contents, [0; 6]);
}

fn activation_system(contents: &str, registers: [i64; 6]) -> i64 {
    let mut device = Device::load(contents, &registers);
    let cnt = device.execute();
    println!("Executed {} steps", cnt);
    device.registers()[0]
}

fn activation_endless(contents: &str, registers: [i64; 6]) -> i64 {
    let mut device = Device::load(contents, &registers);
    let cnt = device.execute();
    println!("Executed {} steps", cnt);
    device.registers()[0]
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn sample_input() {
//        let contents = "#ip 0
//seti 5 0 1
//seti 6 0 2
//addi 0 1 0
//addr 1 2 3
//setr 1 0 0
//seti 8 0 4
//seti 9 0 5";
//
//        assert_eq!(background_process(&contents, [0; 6]), 6);
//    }

}
