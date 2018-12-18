extern crate util;

fn main() {
    let contents = util::string_from_file("input.txt");

    let result1 = simulate(&contents, 50);
    println!("Part 1 Result: {}", result1);
}

fn simulate(contents: &str, size: isize) -> usize {
    let mut grid = Grid::new(&contents, size);
    for _i in 1..=10 {
        grid.step();
    }
    grid.print();
    grid.resource_value()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SquareType {
    Open,
    Trees,
    LumberYard,
    Invalid,
}

impl SquareType {
    fn next(&self, neighbors: &Vec<SquareType>) -> SquareType {
       match self {
           SquareType::Open => SquareType::update_open(neighbors),
           SquareType::Trees => SquareType::update_trees(neighbors),
           SquareType::LumberYard => SquareType::update_lumberyard(neighbors),
           _ => SquareType::Invalid,
       }
    }

    fn update_open(neighbors: &Vec<SquareType>) -> SquareType {
        let filtered: Vec<_> = neighbors.iter().filter(|x| **x == SquareType::Trees).collect();
        if filtered.len() >= 3 {
            SquareType::Trees
        } else {
            SquareType::Open
        }
    }

    fn update_trees(neighbors: &Vec<SquareType>) -> SquareType {
        let filtered: Vec<_> = neighbors.iter().filter(|x| **x == SquareType::LumberYard).collect();
        if filtered.len() >= 3 {
            SquareType::LumberYard
        } else {
            SquareType::Trees
        }
    }

    fn update_lumberyard(neighbors: &Vec<SquareType>) -> SquareType {
        let trees: Vec<_> = neighbors.iter().filter(|x| **x == SquareType::Trees).collect();
        let lumberyards: Vec<_> = neighbors.iter().filter(|x| **x == SquareType::LumberYard).collect();
        if trees.len() >= 1 && lumberyards.len() >= 1 {
            SquareType::LumberYard
        } else {
            SquareType::Open
        }
    }
}

#[derive(Clone)]
struct Grid {
    squares: [[SquareType; 50]; 50],
    size: isize,
}

impl Grid {
    fn new(contents: &str, size: isize) -> Grid {
        let mut grid = Grid {squares: [[SquareType::Invalid; 50]; 50], size: size};
        let mut x;
        let mut y = 0;
        for l in contents.lines() {
            x = 0;
            for c in l.chars() {
                let val = match c {
                    '.' => SquareType::Open,
                    '|' => SquareType::Trees,
                    '#' => SquareType::LumberYard,
                    _ => SquareType::Invalid,
                };
                grid.squares[x][y] = val;
                x += 1;
            }
            y += 1;
        }
        grid
    }

    fn resource_value(&self) -> usize {
        let mut tree_cnt = 0;
        let mut yard_cnt = 0;
        for y in 0..self.size {
            for x in 0..self.size {
                match self.squares[x as usize][y as usize] {
                    SquareType::Trees => tree_cnt += 1,
                    SquareType::LumberYard => yard_cnt += 1,
                    _ => (),
                }
            }
        }
        tree_cnt * yard_cnt
    }

    fn step(&mut self) {
        let mut new = [[SquareType::Invalid; 50]; 50];
        for y in 0..self.size {
            for x in 0..self.size {
                let curr = self.squares[x as usize][y as usize];
                let n = self.neighbors(x, y);
                new[x as usize][y as usize] = curr.next(&n);
            }
        }
        self.squares = new;
    }

    fn neighbors(&self, x_in: isize, y_in: isize) -> Vec<SquareType> {
        let mut v: Vec<SquareType> = Vec::new();
        for y in y_in-1..=y_in+1 {
            for x in x_in-1..=x_in+1 {
                if x == x_in && y == y_in {
                    continue;
                }
                let n = self.get_square(x, y);
                v.push(n);
            }
        }
        v
    }

    fn get_square(&self, x: isize, y: isize) -> SquareType {
        if x < 0 || y < 0 {
            SquareType::Invalid
        } else if x >= self.size || y >= self.size {
            SquareType::Invalid
        } else {
            self.squares[x as usize][y as usize]
        }
    }

    fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let c = 
                    match self.squares[x as usize][y as usize] {
                        SquareType::Open => '.',
                        SquareType::Trees => '|',
                        SquareType::LumberYard => '#',
                        _ => 'x',
                    };
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FILE: &str = "test_input.txt";

    #[test]
    fn test_sample1() {
        let contents = util::string_from_file(TEST_FILE);
        assert_eq!(simulate(&contents, 10), 1147);
    }
}
