extern crate util;

pub fn react_count(s: &str) -> usize {
    let trimmed = s.trim();
    let reacted = react(trimmed);
    reacted.iter().count()
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
}
