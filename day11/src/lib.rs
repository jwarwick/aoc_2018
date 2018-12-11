extern crate util;

pub fn largest_total_power(serial: isize) -> ((usize, usize), isize) {
    let mut cells = [[0isize; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            cells[x][y] = power_level(x, y, serial);
        }
    }

    let mut areas: Vec<(isize, (usize, usize))> = Vec::with_capacity(300 * 300);
    for y in 0..(300 - 2) {
        for x in 0..(300-2) {
            let total =
                cells[x][y] + cells[x+1][y] + cells[x+2][y] +
                cells[x][y+1] + cells[x+1][y+1] + cells[x+2][y+1] +
                cells[x][y+2] + cells[x+1][y+2] + cells[x+2][y+2];

            areas.push((total, (x, y)));
        }
    }

    let (power, (x, y)) = areas.iter().max().expect("Found a max power area");

    ((*x, *y), *power)
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
        assert_eq!(largest_total_power(18), ((33, 45), 29));
        assert_eq!(largest_total_power(42), ((21, 61), 30));
    }
}
