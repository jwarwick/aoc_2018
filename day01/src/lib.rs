extern crate util;
use std::collections::HashSet;

pub fn calibrate_from_file(filename: &str) -> i32 {
    let contents = util::string_from_file(filename);
    //println!("File contents: {}", contents);
    calibrate_from_string(&contents)
}

fn calibrate_from_string(input: &str) -> i32 {
    let nums = util::string_to_numbers(input);
    calibrate(&nums)
}

fn calibrate(input: &Vec<i32>) -> i32 {
    input.iter().sum()
}

pub fn duplicate_from_file(filename: &str) -> i32 {
    let contents = util::string_from_file(filename);
    //println!("File contents: {}", contents);
    let nums = util::string_to_numbers(&contents);
    duplicate_frequency(&nums)
}

fn duplicate_frequency(input: &Vec<i32>) -> i32 {
    let cyc = input.iter().cycle();

    let mut calibration = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(calibration);

    for curr in cyc {
        calibration = calibration + curr;
        if seen.contains(&calibration) {
            break;
        }
        seen.insert(calibration);
    }
    calibration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1a() {
        let input = vec![1, 1, 1];
        assert_eq!(calibrate(&input), 3);
    }

    #[test]
    fn ex1b() {
        let input = vec![1, 1, -2];
        assert_eq!(calibrate(&input), 0);
    }

    #[test]
    fn ex1c() {
        let input = vec![-1, -2, -3];
        assert_eq!(calibrate(&input), -6);
    }

    #[test]
    fn from_string() {
        let input = "-1\n-2\n-3\n";
        assert_eq!(calibrate_from_string(&input), -6);
    }

    #[test]
    fn dup() {
        let input = vec![3, 3, 4, -2, -4];
        assert_eq!(duplicate_frequency(&input), 10);
    }

    #[test]
    fn dup1() {
        let input = vec![1, -1];
        assert_eq!(duplicate_frequency(&input), 0);
    }

    #[test]
    fn dup2() {
        let input = vec![3, 3, 4, -2, -4];
        assert_eq!(duplicate_frequency(&input), 10);
    }

    #[test]
    fn dup3() {
        let input = vec![-6, 3, 8, 5, -6];
        assert_eq!(duplicate_frequency(&input), 5);
    }

    #[test]
    fn dup4() {
        let input = vec![7, 7, -2, -7, -4];
        assert_eq!(duplicate_frequency(&input), 14);
    }
}
