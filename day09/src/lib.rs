extern crate util;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Marble {
    value: u32,
    next: u32,
    prev: u32,
}

fn play(players: &u32, last_value: &u32) -> Vec<u32> {
    println!("Players: {}, Last Value: {}", players, last_value);
    let mut marbles: HashMap<u32, Marble> = HashMap::with_capacity(*last_value as usize + 1);
    let mut curr = Marble {value: 0, next: 0, prev: 0};
    let mut cnt: usize = 0;
    let mut scores: Vec<u32> = vec![0; *players as usize];

    marbles.insert(0, curr);
    cnt += 1;

    while cnt <= *last_value as usize {
        if 0 == cnt % 23 {
            let mut take_val = curr.prev;
            for _ in 0..6 {
                take_val = marbles[&take_val].prev;
            }
            let take = marbles.remove(&take_val).expect("Node to remove");
            {
                let prev_node = marbles.get_mut(&take.prev).unwrap();
                prev_node.next = take.next;
            }
            {
                let next_node = marbles.get_mut(&take.next).unwrap();
                next_node.prev = take.prev;
                curr = next_node.clone();
            }

            let player: usize = ((cnt + 1) % *players as usize);
            scores[player] += (cnt as u32) + take.value;

        } else {
            let next = marbles[&curr.next].next;
            let prev = curr.next;
            let value: u32 = cnt as u32;
            {
                let prev_node = marbles.get_mut(&prev).unwrap();
                prev_node.next = value;
            }
            {
                let next_node = marbles.get_mut(&next).unwrap();
                next_node.prev = value;
            }
            curr = Marble {value, next, prev};
            marbles.insert(cnt as u32, curr);
        }
        
        //println!("{:#?}", marbles);
        cnt += 1;
    }

    scores
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
