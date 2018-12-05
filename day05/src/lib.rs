extern crate util;

use std::collections::HashSet;

pub fn react_count(s: &str) -> usize {
    let reacted = react(s);
    reacted.iter().count()
}

pub fn best_polymer_count(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let uniq: HashSet<char> = chars.iter().map(|x| x.to_ascii_lowercase()).collect();
    let mut sizes: Vec<usize> = Vec::new();
    for u in uniq {
        let u_up = u.to_ascii_uppercase();
        let filtered: Vec<char> = chars.iter().filter(|&x| *x != u && *x != u_up).cloned().collect();
        let r = one_pass(filtered);
        sizes.push(r.iter().count());
    }
    *sizes.iter().min().expect("No minimum")
}

fn react(s: &str) -> Vec<char> {
    let chars: Vec<char> = s.chars().collect();
    one_pass(chars)
}

fn one_pass(input: Vec<char>) -> Vec<char> {
    input
        .iter()
        .fold(Vec::new(), |mut result, c|
              {
                  let prev = result.last().unwrap_or(&'1').clone();
                  if prev != *c && prev.to_ascii_lowercase() == c.to_ascii_lowercase() {
                      result.pop();
                  } else {
                      result.push(c.clone());
                  }
                  result
              })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_match() {
        let input = "aA";
        let result: Vec<char> = Vec::new();
        assert_eq!(result, react(input));
    }

    #[test]
    fn one_match_rev() {
        let input = "Aa";
        let result: Vec<char> = Vec::new();
        assert_eq!(result, react(input));
    }

    #[test]
    fn one_match_rem() {
        let input = "Aac";
        assert_eq!(vec!['c'], react(input));
    }

    #[test]
    fn two_match() {
        let input = "abBA";
        let result: Vec<char> = Vec::new();
        assert_eq!(result, react(input));
    }

    #[test]
    fn no_match_chars() {
        let input = "abAB";
        assert_eq!(vec!['a', 'b', 'A', 'B'], react(input));
    }

    #[test]
    fn no_match_polarity() {
        let input = "aabAAB";
        assert_eq!(vec!['a', 'a', 'b', 'A', 'A', 'B'], react(input));
    }

    #[test]
    fn longer_example() {
        let input = "dabAcCaCBAcCcaDA";
        let result: Vec<char> = "dabCBAcaDA".chars().collect();
        assert_eq!(result, react(input));
    }

    #[test]
    fn longer_count() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(10, react_count(&input));
    }

    #[test]
    fn best_polymer() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(4, best_polymer_count(&input));
    }
}
