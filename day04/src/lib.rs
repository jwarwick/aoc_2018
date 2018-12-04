extern crate util;
//#[macro_use] extern crate itertools;
#[macro_use] extern crate scan_fmt;

use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Nap {
    guard: i32,
    start: i32,
    end: i32,
    duration: i32
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Action {
    FallsAsleep,
    WakesUp,
    StartsShift(i32),
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Event {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    action: Action,
}

impl FromStr for Event {
    type Err = ();
    fn from_str(s: &str) -> Result<Event, ()> {
        //[1518-11-01 00:00] Guard #10 begins shift
        //[1518-11-01 00:05] falls asleep
        //[1518-11-01 00:25] wakes up
        let (y, m, d, h, min, act_s) = scan_fmt!(s,
                                           "[{d}-{d}-{d} {d}:{d}] {[^\n]}",
                                           i32, i32, i32, i32, i32, String);
        let act = parse_action(&act_s.unwrap());

        Ok(Event {
            year: y.unwrap(),
            month: m.unwrap(),
            day: d.unwrap(),
            hour: h.unwrap(),
            min: min.unwrap(),
            action: act,
        })
    }

}

pub fn parse_file(filename: &str) -> Vec<Nap> {
    let contents = util::string_from_file(filename);
    let events = parse_events(&contents);
    create_naps(&events)
}

fn parse_events(string: &str) -> Vec<Event> {
    let mut events: Vec<Event> = string.lines().map(|s| s.parse()).flatten().collect();
    events.sort();
    events
}

fn parse_action(s: &str) -> Action {
    if s.starts_with("falls") {
        Action::FallsAsleep
    } else if s.starts_with("wakes") {
        Action::WakesUp
    } else {
        let num = scan_fmt!(s, "Guard #{d} begins shift", i32);
        Action::StartsShift(num.unwrap())
    }
}

fn create_naps(events: &Vec<Event>) -> Vec<Nap> {
    let mut naps: Vec<Nap> = Vec::new();
    let mut curr_guard = 0;
    let mut curr_start = 0;
    for evt in events {
        match evt.action {
            Action::StartsShift(g) => curr_guard = g,
            Action::FallsAsleep => curr_start = evt.min,
            Action::WakesUp => {
                naps.push(Nap{guard: curr_guard,
                    start: curr_start,
                    end: evt.min,
                    duration: evt.min - curr_start,});
            },
        }
    }
    naps
}

pub fn strategy1(input: &Vec<Nap>) -> usize {
    let mut guards: HashMap<i32, i32> = HashMap::new();
    for nap in input {
        let counter = guards.entry(nap.guard).or_insert(0);
        *counter += nap.duration;
    }
    let (guard, _duration): (&i32, &i32) = guards.iter().max_by(|(_k1, v1), (_k2, v2)| v1.cmp(&v2)).unwrap();
    let guard_naps: Vec<&Nap> = input.iter().filter(|n| *guard == n.guard).collect();
    let mut minutes: [i32; 60] = [0; 60];
    for n in guard_naps {
        for m in n.start .. n.end {
            minutes[m as usize] += 1;
        }
    }
    let (_max_value, max_idx) = minutes.iter().enumerate().map(|(a, b)| (b, a)).max().unwrap();
    max_idx * (*guard as usize)
}

pub fn strategy2(input: &Vec<Nap>) -> usize {
    let mut guards: HashMap<i32, [i32; 60]> = HashMap::new();
    for nap in input {
        let times = guards.entry(nap.guard).or_insert([0; 60]);
        for m in nap.start .. nap.end {
            times[m as usize] += 1;
        }
    }

    let mut max_mins: Vec<(i32, i32, usize)> = Vec::new();
    for (guard_id, times) in guards.iter() {
        let (max_value, max_idx) = times.iter().enumerate().map(|(a, b)| (b, a)).max().unwrap();
        max_mins.push((*max_value, *guard_id, max_idx));
    }
    let (_max_value, guard_id, max_time) = max_mins.iter().max().unwrap();
    max_time * (*guard_id as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_falls() {
        let input = "[1518-05-03 00:30] falls asleep";
        let result: Event = input.parse().unwrap();
        assert_eq!(result, Event{year: 1518, month: 5, day: 3, hour: 0, min: 30, action: Action::FallsAsleep});
    }

    #[test]
    fn parse_wakes() {
        let input = "[1518-11-01 00:25] wakes up";
        let result: Event = input.parse().unwrap();
        assert_eq!(result, Event{year: 1518, month: 11, day: 1, hour: 0, min: 25, action: Action::WakesUp});
    }

    #[test]
    fn parse_shift() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift";
        let result: Event = input.parse().unwrap();
        assert_eq!(result, Event{year: 1518, month: 11, day: 1, hour: 0, min: 0, action: Action::StartsShift(10)});
    }

    #[test]
    fn test_parse_string() {
        let input = "[1518-11-02 00:50] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep";
        let result = parse_events(input);
        assert_eq!(result, vec![
                   Event{year: 1518, month: 11, day: 1, hour: 23, min: 58, action: Action::StartsShift(99)},
                   Event{year: 1518, month: 11, day: 2, hour: 0, min: 40, action: Action::FallsAsleep},
                   Event{year: 1518, month: 11, day: 2, hour: 0, min: 50, action: Action::WakesUp},
        ]);

        let nap = create_naps(&result);
        assert_eq!(nap, vec![Nap{guard: 99, start: 40, end: 50, duration: 10}]);
    }

    #[test]
    fn test_create_naps() {
        let input = "[1519-11-02 00:50] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1519-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1519-11-01 23:58] Guard #12 begins shift\n[1518-11-02 00:40] falls asleep";
        let result = parse_events(input);
        let nap = create_naps(&result);
        assert_eq!(nap, vec![Nap{guard: 99, start: 40, end: 50, duration: 10}, Nap{guard: 12, start: 40, end: 50, duration: 10}]);
    }

    #[test]
    fn test_strat1() {
        let input = parse_file("test_input.txt");
        assert_eq!(240, strategy1(&input));
    }

    #[test]
    fn test_strat2() {
        let input = parse_file("test_input.txt");
        assert_eq!(4455, strategy2(&input));
    }
}
