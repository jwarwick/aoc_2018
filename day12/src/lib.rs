extern crate util;

use std::collections::HashMap;
use std::collections::HashSet;

type Rule = (bool, bool, bool, bool, bool);
type Rules = HashSet<Rule>;
type State = HashMap<isize, bool>;

pub fn plant_sum(contents: &str) -> isize {
    let mut rules: Rules = HashSet::new();
    let mut state: State = HashMap::new();
    parse(contents, &mut rules, &mut state);
    print_state(&state);

    for _s in 0..20 {
        state = step(state, &rules);
        print_state(&state);
    }
    sum_state(&state)
}

fn sum_state(state: &State) -> isize {
    let mut total = 0;
    for (idx, val) in state.iter() {
        if *val {
            total += idx;
        }
    }
    total
}

fn step(state: State, rules: &Rules) -> State {
    let mut new_state: State = HashMap::new();
    for k in state.keys() {
        update_pot(k, rules, &state, &mut new_state);
    }
    new_state
}

fn update_pot(idx: &isize, rules: &Rules, state: &State, new_state: &mut State) {
    let a = get_or_update_pot(*idx - 2, state, new_state);
    let b = get_or_update_pot(*idx - 1, state, new_state);
    let c = get_or_update_pot(*idx + 0, state, new_state);
    let d = get_or_update_pot(*idx + 1, state, new_state);
    let e = get_or_update_pot(*idx + 2, state, new_state);

    new_state.insert(*idx, rules.contains(&(a, b, c, d, e)));
}

fn get_or_update_pot(idx: isize, state: &State, new_state: &mut State) -> bool {
    let val = state.get(&idx);
    match val {
        None => {
            new_state.insert(idx, false);
            false
        },
        Some(x) => *x
    }
}

fn print_state(state: &State) {
    let mut sorted: Vec<_> = state.iter().collect();
    sorted.sort();
    for (_i, val) in sorted {
        print!("{}", if *val {'#'} else {'.'});
    }
    print!("\n");
}

fn parse(contents: &str, rules: &mut Rules, state: &mut State) {
    let mut lines = contents.lines();
    let state_str = lines.next().expect("Found state string");
    let state_chars = state_str.to_string().split_off(15);
    for (idx, c) in state_chars.chars().enumerate() {
        state.insert(idx as isize, c == '#');
    }

    lines.next();
    for l in lines {
        let chars: Vec<_> = l.chars().collect();
        if chars[9] == '#' {
            let r = (chars[0] == '#',
                     chars[1] == '#',
                     chars[2] == '#',
                     chars[3] == '#',
                     chars[4] == '#',);
            rules.insert(r);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let filename = "test_input.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(plant_sum(&contents), 325);
    }
}
