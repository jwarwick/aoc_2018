extern crate util;
extern crate itertools;

use itertools::Itertools;

pub fn parse_file(filename: &str) -> Vec<Vec<char>> {
    let contents = util::string_from_file(filename);
    let contents = contents.trim();
    contents
        .split_whitespace()
        .map(|w| w.chars().collect())
        .collect()
}

pub fn checksum(input: &Vec<Vec<char>>) -> usize {
    let cnts: Vec<Vec<(char, i32)>> = input.iter().map(char_counts).collect();
    let twos = cnts.iter().filter(|v| has_num(v, 2)).count();
    let threes = cnts.iter().filter(|v| has_num(v, 3)).count();
    twos * threes
}

pub fn common_letters(input: &Vec<Vec<char>>) -> String {
    let close = close_ids(&input);
    let intersect = id_intersection(close);
    let s: String = intersect.iter().collect();
    s
}

fn char_counts(input: &Vec<char>) -> Vec<(char, i32)> {
    let mut s = input.clone();
    s.sort();
    let start: Vec<(char, i32)> = s.iter().map(|c| (c.clone(), 1)).collect();
    start.into_iter().coalesce(|(c1, cnt1), (c2, cnt2)|
                               if c1 == c2 {
                                   Ok((c1, cnt1 + cnt2))
                               } else {
                                   Err(((c1, cnt1), (c2, cnt2)))
                               }).collect()
}

fn has_num(v: &Vec<(char, i32)>, num: i32) -> bool {
    v.iter().any(|&(_c, n)| n == num)
}

fn id_intersection((x, y): (Vec<char>, Vec<char>)) -> Vec<char> {
     x.iter()
         .zip(y)
         .filter(|(a, b)| *a == b)
         .map(|(a, _b)| *a)
         .collect()
}

fn close_ids(input: &Vec<Vec<char>>) -> (Vec<char>, Vec<char>) {
    let mut result = (vec![], vec![]);
    for v in input {
        let mut r = input.iter().filter(|e| is_close(v, e));
        match r.nth(0) {
            Some(x) =>
            {
                result = (v.to_vec(), x.to_vec());
                //println!("result: {:?}", result);
                break
            },
            None =>
                (),
        }
    }
    result
}

fn is_close(a: &Vec<char>, b: &Vec<char>) -> bool {
    1 == id_diff(&a, &b)
}

fn id_diff(a: &Vec<char>, b: &Vec<char>) -> usize {
     a.iter()
         .zip(b)
         .filter(|(x, y)| x != y)
         .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_three() {
        let input = "bababc";
        let chars = input.chars().collect();
        assert_eq!(char_counts(&chars), vec![('a', 2), ('b', 3), ('c', 1)]);
    }

    #[test]
    fn has_two_three() {
        let input = "bababc";
        let chars = input.chars().collect();
        let cnts = char_counts(&chars);
        assert_eq!(has_num(&cnts, 1), true);
        assert_eq!(has_num(&cnts, 2), true);
        assert_eq!(has_num(&cnts, 3), true);
        assert_eq!(has_num(&cnts, 7), false);
    }

    #[test]
    fn sample_input() {
        let input = parse_file("test_input.txt");
        assert_eq!(checksum(&input), 12);
    }

    #[test]
    fn delta_count() {
        let a: Vec<char> = "abcde".chars().collect();
        let b: Vec<char> = "axcye".chars().collect();
        assert_eq!(id_diff(&a, &b), 2);
        assert_eq!(is_close(&a, &b), false);
    }

    #[test]
    fn delta_count2() {
        let a: Vec<char> = "fghij".chars().collect();
        let b: Vec<char> = "fghuj".chars().collect();
        assert_eq!(id_diff(&a, &b), 1);
        assert_eq!(is_close(&a, &b), true);
    }

    #[test]
    fn diff_input() {
        let input = parse_file("test_input_2.txt");
        let close = close_ids(&input);
        assert_eq!(close, (vec!['f', 'g', 'h', 'i', 'j'], vec!['f', 'g', 'u', 'i', 'j']));
        let intersect = id_intersection(close);
        assert_eq!(intersect, vec!['f', 'g', 'i', 'j']);
    }
}
