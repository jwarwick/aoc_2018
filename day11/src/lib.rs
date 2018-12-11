extern crate util;

pub fn largest_total_power_any(serial: isize) -> ((usize, usize, usize), isize) {
    let mut areas: Vec<(isize, (usize, usize, usize))> = Vec::with_capacity(300);
    for size in 1..300 {
        let (coord, power) = largest_total_power(serial, size);
        areas.push((power, coord));
        if power < 0 {
            break;
        }
    }
    let (power, (x, y, found_size)) = areas.iter().max().expect("Found a variable sized max power area");
    ((*x, *y, *found_size), *power)
}

pub fn largest_total_power(serial: isize, size: usize) -> ((usize, usize, usize), isize) {
    let mut cells = [[0isize; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            cells[x][y] = power_level(x, y, serial);
        }
    }

    let mut areas: Vec<(isize, (usize, usize, usize))> = Vec::with_capacity(300 * 300);
    for y in 0..(300 - (size - 1)) {
        for x in 0..(300 - (size - 1)) {
            let mut total: isize = 0;
            for h in 0..size {
                for w in 0..size {
                    total += cells[x+w][h+y];
                }
            }

            areas.push((total, (x, y, size)));
        }
    }

    let (power, (x, y, found_size)) = areas.iter().max().expect("Found a max power area");

    ((*x, *y, *found_size), *power)
}

fn power_level(x: usize, y: usize, serial: isize) -> isize {
    let rack_id: isize = x as isize + 10;
    let mut power_level: isize = rack_id * (y as isize);
    power_level += serial;
    power_level *= rack_id;
    power_level = power_level % 1000;
    power_level = power_level/100;
    power_level - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_levels() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_largest_area() {
        assert_eq!(largest_total_power(18, 3), ((33, 45, 3), 29));
        assert_eq!(largest_total_power(42, 3), ((21, 61, 3), 30));
    }

    #[test]
    fn test_largest_variable_area() {
        assert_eq!(largest_total_power_any(18), ((90, 269, 16), 113));
    }

    #[test]
    fn test_largest_variable_area2() {
        assert_eq!(largest_total_power_any(42), ((232, 251, 12), 119));
    }
}
