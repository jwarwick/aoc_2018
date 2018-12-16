extern crate util;

use std::collections::HashMap;
use std::collections::HashSet;

const ATTACK_POWER: isize = 3;

type Map = HashMap<(usize, usize), Entry>;
type Loc = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Entry {
    Empty,
    Wall,
    Goblin{hp: isize},
    Elf{hp: isize},
}

fn print_map(map: &Map) {
    let keys = map.keys();
    let (max_x, _y) = keys.clone().max_by(|(x1, _y1), (x2, _y2)| x1.cmp(x2)).expect("Max x value");
    let (_x, max_y) = keys.max_by(|(_x1, y1), (_x2, y2)| y1.cmp(y2)).expect("Max y value");
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            let c = match map.get(&(x, y)).or(Some(&Entry::Empty)).unwrap() {
                Entry::Wall => '#',
                Entry::Goblin{hp: _hp} => 'G',
                Entry::Elf{hp: _hp} => 'E',
                _ => '.',
            };
            print!("{}", c);
        }
        print!("\n");
    }
}

fn print_health(map: &Map) {
    for x in map.iter() {
        match x {
            (_loc, Entry::Goblin{hp: _h}) => println!("{:?}", x),
            (_loc, Entry::Elf{hp: _h}) => println!("{:?}", x),
            _ => (),
        }
    }
}

fn parse(contents: &str, map: &mut Map) {
    let lines = contents.lines();
    for (y, l) in lines.enumerate() {
        for (x, c) in l.chars().enumerate() {
            let val = match c {
                '#' => Entry::Wall,
                'G' => Entry::Goblin{hp: 200},
                'E' => Entry::Elf{hp: 200},
                _ => Entry::Empty,
            };
            if val != Entry::Empty {
                map.insert((x, y), val);
            }
        }
    }
}

fn reading_order(points: &Vec<Loc>) -> Vec<Loc> {
    let mut flipped: Vec<_> =
        points
        .iter()
        .map(|(x, y)| (y, x))
        .collect();
    flipped.sort();
    let sorted_flipped: Vec<_> =
        flipped
        .iter()
        .cloned()
        .map(|(y, x)| (x.clone(), y.clone()))
        .collect();
    sorted_flipped
}

fn initiative(map: &Map) -> Vec<Loc> {
    let actors: Vec<_> = map
        .iter()
        .filter(|(_pos, t)|
                match **t  {
                    Entry::Elf{hp: _hp} => true,
                    Entry::Goblin{hp: _hp} => true,
                    _ => false,
                }
        )
    .map(|((x, y), _t)| (x.clone(), y.clone()))
        .collect();
    reading_order(&actors)
}

fn enemy(me: &Entry) -> Option<Entry> {
    match me {
        Entry::Elf{hp: _hp} => Some(Entry::Goblin{hp: 0}),
        Entry::Goblin{hp: _hp} => Some(Entry::Elf{hp: 0}),
        _ => None,
    }
}

fn get_elves(map: &Map) -> Option<Vec<Loc>> {
    let actors: Vec<_> = map
        .iter()
        .filter(|(_pos, t)|
                match **t  {
                    Entry::Elf{hp: _hp} => true,
                    _ => false,
                }
        )
    .map(|((x, y), _t)| (x.clone(), y.clone()))
        .collect();
    if actors.is_empty() {
        None
    } else {
        Some(actors)
    }
}

fn get_goblins(map: &Map) -> Option<Vec<Loc>> {
    let actors: Vec<_> = map
        .iter()
        .filter(|(_pos, t)|
                match **t  {
                    Entry::Goblin{hp: _hp} => true,
                    _ => false,
                }
        )
    .map(|((x, y), _t)| (x.clone(), y.clone()))
        .collect();
    if actors.is_empty() {
        None
    } else {
        Some(actors)
    }
}

fn enemies(loc: Loc, map: &Map) -> Option<Vec<Loc>> {
    let me = map.get(&loc);
    match me {
        None => None,
        Some(x) => {
            let enemy_type = enemy(x);
            match enemy_type {
                None => None,
                Some(Entry::Elf{hp: _hp}) => get_elves(&map),
                Some(Entry::Goblin{hp: _hp}) => get_goblins(&map),
                _ => None,
            }
        }
    }
}

fn adjacent((mx, my): &Loc, points: &Vec<Loc>) -> Vec<Loc> {
    let mut adj: Vec<Loc> = Vec::new();
    for (x, y) in points {
        if *mx == *x && (*my == (y+1) || *my == (y-1)) {
            adj.push((*x,*y));
        } else if *my == *y && (*mx == (x+1) || *mx == (x-1)) {
            adj.push((*x,*y));
        }
    }
    adj
}

fn is_occupied(point: &Loc, map: &Map) -> bool {
    match map.get(&point) {
        None => false,
        _ => true
    }
}

fn open_squares(points: &Vec<Loc>, map: &Map) -> HashSet<Loc> {
    let mut open: HashSet<Loc> = HashSet::new();
    for (x, y) in points {
       if !is_occupied(&(*x, y-1), &map) {
           open.insert((x.clone(), y-1));
       }
       if !is_occupied(&(*x, y+1), &map) {
           open.insert((x.clone(), y+1));
       }
       if !is_occupied(&(x-1, *y), &map) {
           open.insert((x-1, y.clone()));
       }
       if !is_occupied(&(x+1, *y), &map) {
           open.insert((x+1, y.clone()));
       }
    }
    open
}

fn distance_recurse(points: &mut Vec<(Loc, usize)>, distances: &mut HashMap<Loc, usize>, map: &Map) {
    while !points.is_empty() {
        //if points.is_empty() {
        //    return;
        //}

        let (pt, dist) = points.pop().expect("A point to check");
        {
            let curr = distances.entry(pt).or_insert(dist);
            if dist < *curr {
                *curr = dist;
            }
        }

        let neighbors = open_squares(&vec![pt], &map);
        for n in neighbors {
            match distances.get(&n) {
                None => points.push((n, dist+1)),
                Some(x) => 
                {
                    if *x > dist + 1 {
                        points.push((n, dist+1));
                    }
                },
            }
        }
    }

    //distance_recurse(points, distances, map);
}

fn distance_map(loc: &Loc, map: &Map) -> HashMap<Loc, usize> {
    let mut distances: HashMap<Loc, usize> = HashMap::new();
    let mut points = vec![(*loc, 0)];
    distance_recurse(&mut points, &mut distances, &map);
    distances
}

fn total_hit_points(map: &Map) -> usize {
    let mut total: usize = 0;
    for (_loc, t) in map.iter() {
        match t {
            Entry::Elf{hp: h} => total += *h as usize,
            Entry::Goblin{hp: h} => total += *h as usize,
            _ => (),
        };
    }

    total
}

pub fn simulate(contents: &str) -> usize {
    let mut map: Map = HashMap::new();
    parse(contents, &mut map);
    print_map(&map);
    let mut last_round: usize = 0;

    'outer: for round in 1..200 {
        let mut killed: Vec<Loc> = Vec::new();
        'inner: for x in initiative(&map) {
            if killed.contains(&x) {
                continue;
            }
            let mut chosen_step = x;
            let enemies = enemies(x, &map);
            //println!("me: {:?}, enemies: {:?}", x, enemies);
            if None == enemies {
                //println!("No enemies left");
                last_round = round - 1;
                break 'outer;
            }
            let enemies = enemies.expect("Enemies left");
            let adjacent_enemies = adjacent(&x, &enemies);
            //println!("Adjacent: {:?}", adjacent_enemies);
            if adjacent_enemies.is_empty() {
                let mut in_range_distances: HashMap<Loc, usize> = HashMap::new();
                {
                    let in_range = open_squares(&enemies, &map);
                    //println!("In range: {:?}", in_range);
                    let adj_distance_map = distance_map(&x, &map);
                    //println!("Distances: {:?}", adj_distance_map);
                    for r in in_range {
                        match adj_distance_map.get(&r) {
                            None => None,
                            Some(dist) => in_range_distances.insert(r, dist.clone()),
                        };
                    }
                }
                //println!("Reachable: {:?}", in_range_distances);
                if in_range_distances.is_empty() {
                    continue;
                }
                
                let mut chosen: Loc;
                {
                    let min_dist = in_range_distances.values().min().expect("Min distance");
                    let nearest: Vec<_> =
                        in_range_distances
                        .iter()
                        .filter(|(_loc, dist)| *dist == min_dist)
                        .map(|(loc, _dist)| loc.clone())
                        .collect();
                    //println!("Min: {}", min_dist);
                    //println!("Nearest: {:?}", nearest);
                    let nearest_sort = reading_order(&nearest);
                    chosen = nearest_sort.first().expect("Chosen node").clone();
                    //println!("Chosen: {:?}", chosen);
                }

                let chosen_distance_map = distance_map(&chosen, &map);
                let step_options = open_squares(&vec![x], &map);
                let mut step_distances: HashMap<Loc, usize> = HashMap::new();
                for s in step_options {
                    match chosen_distance_map.get(&s) {
                        None => None,
                        Some(dist) => step_distances.insert(s, *dist),
                    };
                }
                //println!("Step distances: {:?}", step_distances);
                let step_min_dist = step_distances.values().min().expect("Step min distance");
                //println!("Min dist: {}", step_min_dist);
                let step_nearest: Vec<_> =
                    step_distances
                    .iter()
                    .filter(|(_loc, dist)| *dist == step_min_dist)
                    .map(|(loc, _dist)| loc.clone())
                    .collect();
                let step_nearest_sort = reading_order(&step_nearest);
                chosen_step = *step_nearest_sort.first().expect("Step chosen node");
                //println!("Step to take: {:?}", chosen_step);
                let actor = map.remove(&x).expect("Current node");
                map.insert(chosen_step, actor);
            }

            let adjacent_enemies = adjacent(&chosen_step, &enemies);
            let mut enemy_list: Vec<(Loc, Entry)> = Vec::new();
            let mut min_health = 500;
            if !adjacent_enemies.is_empty() {
                //println!("Adjacent: {:?}", adjacent_enemies);
                for e in adjacent_enemies {
                    let e_entry = map.get(&e).expect("Adjacent enemy");
                    let curr_health = match e_entry {
                        Entry::Elf{hp} => hp,
                        Entry::Goblin{hp} => hp,
                        _ => &1000,
                    };
                    if *curr_health < min_health {
                        min_health = *curr_health;
                    }

                    enemy_list.push((e, *e_entry));
                }
                //println!("Enemy list: {:?}", enemy_list);
                //println!("Min health: {}", min_health);
                let min_health_enemies: Vec<Loc> =
                    enemy_list
                    .iter()
                    .cloned()
                    .filter(|(_loc, h)|
                            {
                                match h {
                                    Entry::Elf{hp} => *hp == min_health,
                                    Entry::Goblin{hp} => *hp == min_health,
                                    _ => false,
                                }})
                .map(|(loc, _h)| loc)
                    .collect();
                //println!("Min health enemies: {:?}", min_health_enemies);
                let min_health_sort = reading_order(&min_health_enemies);
                let chosen_enemy = min_health_sort.first().expect("Lowest health enemy");
                //println!("Chosen enemy: {:?}", chosen_enemy);
                let new_health = min_health - ATTACK_POWER;
                //println!("new health = {}", new_health);
                if new_health <= 0 {
                    map.remove(chosen_enemy);
                    killed.push(*chosen_enemy);
                } else {
                    let mut bad_guy = map.entry(*chosen_enemy).or_insert(Entry::Wall);
                    //println!("Bad guy: {:?}", *bad_guy);
                    *bad_guy = match *bad_guy {
                        Entry::Elf{hp: h} => Entry::Elf{hp: h - ATTACK_POWER},
                        Entry::Goblin{hp: h} => Entry::Goblin{hp: h - ATTACK_POWER},
                        x => x,
                    };
                }
            }
        }
        //println!("\nAfter {} rounds", round);
        //print_map(&map);
        //print_health(&map);
    }

    println!("\n\nLast Round: {}", last_round);
    let total = total_hit_points(&map);
    println!("Total hit points: {}", total);
    total * last_round
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn one_step() {
    //    let filename = "one_step.txt";
    //    let contents = util::string_from_file(&filename);
    //    assert_eq!(simulate(&contents), 1);
    //}

    //#[test]
    //fn larger_step() {
    //    let filename = "larger_input.txt";
    //    let contents = util::string_from_file(&filename);
    //    assert_eq!(simulate(&contents), 1);
    //}

    #[test]
    fn ordering_test() {
        let filename = "ordering.txt";
        let contents = util::string_from_file(&filename);
        let mut map: Map = HashMap::new();
        parse(&contents, &mut map);
        print_map(&map);
        let result = vec![(2, 1), (4, 1), (1, 2), (3, 2), (5, 2), (2, 3), (4, 3)];
        assert_eq!(initiative(&map), result);
    }

    //#[test]
    //fn open_cells_test() {
    //    let filename = "move_test.txt";
    //    let contents = util::string_from_file(&filename);
    //    assert_eq!(simulate(&contents), 1);
    //}

    #[test]
    fn sample_combat() {
        let filename = "sample_combat.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents), 27730);
    }

    #[test]
    fn combat1() {
        let filename = "combat1.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents), 36334);
    }

    #[test]
    fn combat2() {
        let filename = "combat2.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents), 39514);
    }

    #[test]
    fn combat3() {
        let filename = "combat3.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents), 27755);
    }

    #[test]
    fn combat4() {
        let filename = "combat4.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents), 28944);
    }

    #[test]
    fn combat5() {
        let filename = "combat5.txt";
        let contents = util::string_from_file(&filename);
        assert_eq!(simulate(&contents), 18740);
    }

    //#[test]
    //fn part1_test() {
    //    let filename = "input.txt";
    //    let contents = util::string_from_file(&filename);
    //    assert_eq!(simulate(&contents), 7);
    //}
}
