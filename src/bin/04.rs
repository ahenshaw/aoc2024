use std::collections::HashMap;

advent_of_code::solution!(4);

type Grid = HashMap<(i32, i32), char>;
fn make_grid(input: &str) -> (Grid, usize, usize) {
    let mut grid = Grid::new();

    let rows = input.lines().count();
    let cols = input.len() / rows;
    for (col, line) in input.lines().enumerate() {
        for (row, ch) in line.chars().enumerate() {
            grid.insert((row as i32, col as i32), ch);
        }
    }
    (grid, rows, cols)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, rows, cols) = make_grid(input);
    let dirs = [
        (1i32, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for (dr, dc) in dirs {
                let mut cr = row as i32;
                let mut cc = col as i32;
                for ch in ['X', 'M', 'A', 'S'] {
                    let Some(&gc) = grid.get(&(cr, cc)) else {
                        break;
                    };
                    if gc != ch {
                        break;
                    }
                    if ch == 'S' {
                        count += 1
                    }
                    cr += dr;
                    cc += dc;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, rows, cols) = make_grid(input);
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            let row = row as i32;
            let col = col as i32;
            if grid.get(&(row, col)) != Some(&'A') {
                continue;
            }
            let Some(&ul) = grid.get(&(row - 1, col - 1)) else {
                continue;
            };
            let Some(&ur) = grid.get(&(row - 1, col + 1)) else {
                continue;
            };
            let Some(&ll) = grid.get(&(row + 1, col - 1)) else {
                continue;
            };
            let Some(&lr) = grid.get(&(row + 1, col + 1)) else {
                continue;
            };
            if ((ul == 'M' && lr == 'S') || (ul == 'S' && lr == 'M'))
                && ((ll == 'M' && ur == 'S') || (ll == 'S' && ur == 'M'))
            {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
