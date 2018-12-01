extern crate util;

pub fn calibrate_from_file(filename: &str) -> i32 {
    let contents = util::string_from_file(filename);
    println!("File contents: {}", contents);
    calibrate_from_string(&contents)
}

fn calibrate_from_string(input: &str) -> i32 {
    let nums = util::string_to_numbers(input);
    calibrate(&nums)
}

fn calibrate(input: &Vec<i32>) -> i32 {
    input.iter().sum()
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
}
