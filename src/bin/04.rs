advent_of_code::solution!(4);
use advent_of_code::{Grid, NEIGHBORS};

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = input.chars().collect();
    let mut count = 0;

    for start_y in 0..grid.height {
        for start_x in 0..grid.width {
            for (dx, dy) in NEIGHBORS {
                let mut x = start_x as isize;
                let mut y = start_y as isize;
                for ch in "XMAS".chars() {
                    if grid.get_xy(x, y) != Some(&ch) {
                        break;
                    }
                    if ch == 'S' {
                        count += 1
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid<char> = input.chars().collect();
    let mut count = 0;

    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize {
            if grid.get_xy(x, y) != Some(&'A') {
                continue;
            }
            let Some(&ul) = grid.northwest(x, y) else {
                continue;
            };
            let Some(&ur) = grid.northeast(x, y) else {
                continue;
            };
            let Some(&ll) = grid.southwest(x, y) else {
                continue;
            };
            let Some(&lr) = grid.southeast(x, y) else {
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
