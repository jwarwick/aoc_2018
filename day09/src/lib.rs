extern crate util;

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Game {
    marbles: VecDeque<u32>,
    curr: usize,
    scores: Vec<u32>,
}

fn play(players: &u32, last_value: &u32) -> Vec<u32> {
    let mut marbles: VecDeque<u32> = VecDeque::with_capacity(1 + *last_value as usize);
    let mut curr: usize = 0;
    let mut cnt: usize = 0;
    let mut scores: Vec<u32> = vec![0; *players as usize];

    marbles.insert(0, 0);
    cnt += 1;
    println!("{}:\t{:?}", curr, marbles);

    while cnt <= *last_value as usize {
        let marble_cnt = marbles.len();
        if 0 == cnt % 23 {
            let mut take_idx: isize = curr as isize - 7;
            if take_idx < 0 {
                take_idx = (marble_cnt as isize) + take_idx;
            }
            let value = marbles.remove(take_idx as usize).expect("Removable item");
            let player: usize = ((cnt + 1) % *players as usize);
            scores[player] += (cnt as u32) + value;

            curr = take_idx as usize;
            if curr > marbles.len() {
                curr = curr - marbles.len();
            }
        } else {
            let mut new_idx = curr + 2;
            if new_idx > marble_cnt {
                new_idx = new_idx - marble_cnt;
            }
            marbles.insert(new_idx, cnt as u32);
            curr = new_idx;
        }
        
        //print_state(&curr, &marbles);
        cnt += 1;
    }

    scores
}

fn print_state(curr: &usize, marbles: &VecDeque<u32>) {
    for (idx, val) in marbles.iter().enumerate() {
        if *curr == idx {
            print!("({}) ", val);
        } else {
            print!("{} ", val);
        }
    }
    print!("\n");
}

pub fn high_score(players: u32, last_value: u32) -> u32 {
    let scores = play(&players, &last_value);
    *scores.iter().max().expect("A high score")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(high_score(9, 25), 32);

    }
    #[test]
    fn part1_examples() {
        assert_eq!(high_score(10, 1618), 8317);
        assert_eq!(high_score(13, 7999), 146373);
        assert_eq!(high_score(17, 1104), 2764);
        assert_eq!(high_score(21, 6111), 54718);
        assert_eq!(high_score(30, 5807), 37305);
    }
}
