extern crate util;

use std::collections::VecDeque;

pub fn next_ten(cnt: &usize) -> String {
    let mut scores: Vec<usize> = Vec::with_capacity(cnt + 10 + 2);
    let mut elf1 = 0;
    let mut elf2 = 1;
    scores.push(3);
    scores.push(7);
    for _i in 0..*cnt + 10 {
        let score1 = scores[elf1];
        let score2 = scores[elf2];
        let total = score1 + score2;
        let mut digits: Vec<usize> =
            total
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("Found non-digit in string") as usize)
            .collect();
        scores.append(&mut digits);
        elf1 = (elf1 + scores[elf1] + 1) % scores.len();
        elf2 = (elf2 + scores[elf2] + 1) % scores.len();
    }
    let s: String = String::new();
    let result = &scores[*cnt..(*cnt+10)];
    result.iter().fold(s, |mut acc, v| {acc.push_str(&v.to_string()); acc})
}

pub fn recipes_to(input: &str) -> usize {
    let target_vec: VecDeque<usize> =
        input
        .chars()
        .map(|c| c.to_digit(10).expect("Found non-digit in string") as usize)
        .collect();
    let num_digits = target_vec.len();

    let mut scores: Vec<usize> = Vec::new();
    let mut elf1 = 0;
    let mut elf2 = 1;
    scores.push(3);
    scores.push(7);

    let mut last_n: VecDeque<usize> = scores.iter().cloned().collect();

    'outer: for _i in 0.. {
        let score1 = scores[elf1];
        let score2 = scores[elf2];
        let total = score1 + score2;
        let mut digits: Vec<usize> =
            total
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("Found non-digit in string") as usize)
            .collect();
        for d in digits {
            scores.push(d);
            last_n.push_back(d);
            if last_n.len() > num_digits {
                last_n.pop_front();
            }
            if target_vec == last_n {
                break 'outer;
            }
        }
        elf1 = (elf1 + scores[elf1] + 1) % scores.len();
        elf2 = (elf2 + scores[elf2] + 1) % scores.len();
    }

    scores.len() - num_digits
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_9() {
        let input = 9;
        assert_eq!(next_ten(&input), "5158916779");
    }

    #[test]
    fn test_input_5() {
        let input = 5;
        assert_eq!(next_ten(&input), "0124515891");
    }

    #[test]
    fn test_input_18() {
        let input = 18;
        assert_eq!(next_ten(&input), "9251071085");
    }

    #[test]
    fn test_input_2018() {
        let input = 2018;
        assert_eq!(next_ten(&input), "5941429882");
    }

    #[test]
    fn inverse_51589() {
        let input = "51589";
        assert_eq!(recipes_to(&input), 9);
    }

    #[test]
    fn inverse_01245() {
        let input = "01245";
        assert_eq!(recipes_to(&input), 5);
    }

    #[test]
    fn inverse_92510() {
        let input = "92510";
        assert_eq!(recipes_to(&input), 18);
    }

    #[test]
    fn inverse_59414() {
        let input = "59414";
        assert_eq!(recipes_to(&input), 2018);
    }

    #[test]
    fn inverse_15891() {
        let input = "15891";
        assert_eq!(recipes_to(&input), 10);
    }
}
