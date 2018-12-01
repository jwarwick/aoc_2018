use std::fs;
use std::env;

pub fn get_argument(default: &str) -> String {
    let args: Vec<String> = env::args().collect();
    let default = String::from(default);
    args.get(1)
        .unwrap_or(&default)
        .to_string()
}

pub fn string_from_file(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the input file")
}

pub fn string_to_digits(input: &str) -> Vec<u32> {
    let input = input.trim();
    let digits: Vec<_> = input.chars().map(|c| c.to_digit(10).expect("Found non-digit in string")).collect();
    digits
}

pub fn string_to_numbers(input: &str) -> Vec<u32> {
    let input = input.trim();
    let mut output: Vec<u32> = Vec::new();
    let words = input.split_whitespace();
    for word in words {
        let num: u32 = word.parse().expect("Couldn't parse number");
        output.push(num);
    }
    output
}

pub fn string_to_numbers_vec(input: &str) -> Vec<Vec<u32>> {
    let input = input.trim();
    let mut output: Vec<Vec<u32>> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let v = string_to_numbers(line);
        output.push(v);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_dgts() {
        let input = "123";
        assert_eq!(vec![1, 2, 3], string_to_digits(input))
    }

    #[test]
    fn str_to_numbers() {
        let input = "1 2 3";
        assert_eq!(vec![1, 2, 3], string_to_numbers(input))
    }

    #[test]
    fn str_to_vec() {
        let input = "1 2 3\n4 5 6";
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6]], string_to_numbers_vec(input))
    }
}
