extern crate util;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    let armies = build_armies();
    let result1 = remaining_units(&armies);
    println!("Part 1 Result: {}", result1);

//    let result2 = dist_to_center(&content);
//    println!("Part 2 Result: {}", result2);
}

fn remaining_units((immune, infection): &(Army, Army)) -> usize {
    let mut immc = immune.clone();
    let mut infc = infection.clone();
    immc.append(&mut infc);
    let mut all: HashMap<isize, Group> = immc.iter().cloned().map(|a| (a.initiative, a)).collect();

    let mut rem_immune = get_groups(Side::Immune, &all);
    let mut rem_infect = get_groups(Side::Infection, &all);

    while rem_immune.len() != 0 && rem_infect.len() != 0 {
        let order = selection_order(&all);
        let mut attacks: HashMap<isize, isize> = HashMap::new();

        for i in order {
            let curr = &all[&i];
            let mut max_damage = std::isize::MIN;
            let mut best: Option<&Group> = None;
            let mut enemies = if curr.side == Side::Immune {
                &mut rem_infect
            } else {
                &mut rem_immune
            };
            for enemy_idx in enemies.iter() {
                let enemy = &all[enemy_idx];
                let dmg = curr.potential_damage(enemy);
                println!("{:?} {} {} would deal {:?} {} {}  {} damage", curr.side, curr.units, curr.initiative,
                         enemy.side, enemy.units, enemy.initiative, dmg);
                if dmg > max_damage {
                    max_damage = dmg;
                    best = Some(enemy);
                } else if dmg == max_damage {
                    let b = best.expect("Previous best enemy");
                    let b_effective = b.effective_power();
                    let e_effective = enemy.effective_power();
                    if e_effective > b_effective {
                        max_damage = dmg;
                        best = Some(enemy);
                    } else if e_effective == b_effective {
                        if enemy.initiative > b.initiative {
                            max_damage = dmg;
                            best = Some(enemy);
                        }
                    }
                }
            }

            if max_damage > 0 {
                let k = best.expect("Best enemy to attack");
                attacks.insert(curr.initiative, k.initiative);
                enemies.remove(&k.initiative);
            }
        }

        let initiative = initiative_order(&all);
        for i in initiative {
            let mut delete: Option<isize> = None;
            match attacks.get(&i) {
                None => continue,
                Some(e) =>
                {
                    let currw = all.get(&i).cloned();
                    match currw {
                        None => continue,
                        Some(curr) =>
                        {
                            let enemy = match all.entry(*e) {
                                Vacant(_) => continue,
                                Occupied(entry) => entry.into_mut(),
                            };
                            let dmg = curr.potential_damage(&enemy);
                            println!("{:?} {} {} deals {:?} {} {}  {} damage", curr.side, curr.units, curr.initiative,
                                     enemy.side, enemy.units, enemy.initiative, dmg);
                            if enemy.apply_damage(dmg) {
                                delete = Some(enemy.initiative);
                            }
                        },
                    }
                },
            };
            match delete {
                None => continue,
                Some(k) => all.remove(&k),
            };
        }

        rem_immune = get_groups(Side::Immune, &all);
        rem_infect = get_groups(Side::Infection, &all);
        println!("-------------------");
    }

    let s: isize = all.values().map(|a| a.units).sum();
    s as usize
}

fn selection_order(map: &HashMap<isize, Group>) -> Vec<isize> {
    let mut all: Vec<&Group> = map.values().collect();
    all.sort_by(|a, b| a.effective_power().cmp(&b.effective_power()));
    all.reverse();
    let sorted: Vec<isize> = all.iter().map(|a| a.initiative).collect();
    sorted
}

fn initiative_order(map: &HashMap<isize, Group>) -> Vec<isize> {
    let mut all: Vec<&Group> = map.values().collect();
    all.sort_by(|a, b| b.initiative.cmp(&a.initiative));
    let sorted: Vec<isize> = all.iter().map(|a| a.initiative).collect();
    sorted
}

fn get_groups(s: Side, map: &HashMap<isize, Group>) -> HashSet<isize> {
    map.values().filter(|a| s == a.side).map(|a| a.initiative).collect()
}

fn build_armies() -> (Army, Army) {
    (vec![
     Group::new(Side::Immune, 522, 9327, vec![], vec![], 177, AttackType::Slashing, 14),
     Group::new(Side::Immune, 2801, 3302, vec![], vec![], 10, AttackType::Bludgeoning, 7),
     Group::new(Side::Immune, 112, 11322, vec![], vec![], 809, AttackType::Slashing, 8),
     Group::new(Side::Immune, 2974, 9012, vec![], vec![], 23, AttackType::Slashing, 11),
     Group::new(Side::Immune, 4805, 8717, vec![AttackType::Radiation], vec![], 15, AttackType::Bludgeoning, 5),
     Group::new(Side::Immune, 1466, 2562, vec![], vec![AttackType::Radiation, AttackType::Fire], 17, AttackType::Radiation, 10),
     Group::new(Side::Immune, 2513, 1251, vec![AttackType::Fire], vec![AttackType::Cold], 4, AttackType::Slashing, 3),
     Group::new(Side::Immune, 6333, 9557, vec![], vec![AttackType::Slashing], 14, AttackType::Fire, 9),
     Group::new(Side::Immune, 2582, 1539, vec![], vec![AttackType::Bludgeoning], 5, AttackType::Slashing, 2),
     Group::new(Side::Immune, 2508, 8154, vec![AttackType::Bludgeoning, AttackType::Cold], vec![], 27, AttackType::Bludgeoning, 4),
    ],
    vec![
     Group::new(Side::Infection, 2766, 20953, vec![AttackType::Fire], vec![], 14, AttackType::Radiation, 1),
     Group::new(Side::Infection, 4633, 18565, vec![], vec![AttackType::Cold, AttackType::Slashing], 6, AttackType::Fire, 15),
     Group::new(Side::Infection, 239, 47909, vec![AttackType::Slashing, AttackType::Cold], vec![], 320, AttackType::Slashing, 16),
     Group::new(Side::Infection, 409, 50778, vec![], vec![AttackType::Radiation], 226, AttackType::Fire, 17),
     Group::new(Side::Infection, 1280, 54232, vec![], vec![AttackType::Slashing, AttackType::Fire, AttackType::Bludgeoning], 60, AttackType::Bludgeoning, 13),
     Group::new(Side::Infection, 451, 38251, vec![], vec![AttackType::Bludgeoning], 163, AttackType::Bludgeoning, 6),
     Group::new(Side::Infection, 1987, 37058, vec![], vec![], 31, AttackType::Slashing, 20),
     Group::new(Side::Infection, 1183, 19147, vec![AttackType::Slashing], vec![], 24, AttackType::Fire, 12),
     Group::new(Side::Infection, 133, 22945, vec![AttackType::Slashing], vec![AttackType::Cold, AttackType::Bludgeoning], 287, AttackType::Radiation, 19),
     Group::new(Side::Infection, 908, 47778, vec![], vec![], 97, AttackType::Fire, 18),
    ])
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Side {
    Immune,
    Infection,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AttackType {
    Radiation,
    Bludgeoning,
    Fire,
    Slashing,
    Cold,
}

#[derive(Debug, Clone)]
struct Group {
    side: Side,
    units: isize,
    hit_points: isize,
    weak: HashSet<AttackType>,
    immune: HashSet<AttackType>,
    attack_damage: isize,
    attack_type: AttackType,
    initiative: isize,
}

impl Group {
    fn new(side: Side,
           units: isize, hit_points: isize,
           weak: Vec<AttackType>,
           immune: Vec<AttackType>,
           attack_damage: isize, attack_type: AttackType, initiative: isize) -> Group {
        Group {side, units, hit_points, attack_damage, attack_type, initiative,
        weak: weak.iter().cloned().collect(), immune: immune.iter().cloned().collect()}
    }

    fn apply_damage(&mut self, dmg: isize) -> bool {
        let num = dmg / self.hit_points;
        println!("Kills {} units", num);
        self.units = self.units - num;
        self.units <= 0
    }

    fn effective_power(&self) -> isize {
        self.units * self.attack_damage
    }

    fn potential_damage(&self, attacked: &Group) -> isize {
        self.effective_power() * attacked.damage_multiplier(&self.attack_type)
    }

    fn damage_multiplier(&self, attack: &AttackType) -> isize {
        if self.immune.contains(attack) {
            0
        } else if self.weak.contains(attack) {
            2
        } else {
            1
        }
    }
}

type Army = Vec<Group>;

#[cfg(test)]
mod tests {
    use super::*;

    fn build_sample_data() -> (Army, Army) {
        (vec![
         Group::new(Side::Immune, 17, 5390, vec![AttackType::Radiation, AttackType::Bludgeoning], vec![], 4507, AttackType::Fire, 2),
         Group::new(Side::Immune, 989, 1274, vec![AttackType::Bludgeoning, AttackType::Slashing], vec![AttackType::Fire], 25, AttackType::Slashing, 3),
        ],
         vec![
         Group::new(Side::Infection, 801, 4706, vec![AttackType::Radiation], vec![], 116, AttackType::Bludgeoning, 1),
         Group::new(Side::Infection, 4485, 2961, vec![AttackType::Fire, AttackType::Cold], vec![AttackType::Radiation], 12, AttackType::Slashing, 4),
         ])
    }

    #[test]
    fn radius_test() {
        let armies = build_sample_data();
        assert_eq!(remaining_units(&armies), 5216);
    }

}
