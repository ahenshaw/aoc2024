
advent_of_code::solution!(4);
use advent_of_code::Grid;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);

    let dirs: [(isize, isize); 8] = [
        (1, 0),(1, 1),(0, 1),(-1, 1),(-1, 0),(-1, -1),(0, -1),(1, -1),
    ];
    let mut count = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            for (dr, dc) in dirs {
                let mut cr = row as isize;
                let mut cc = col as isize;
                for ch in "XMAS".chars() {
                    let Some(&gc) = grid.get(cr, cc) else {break;};
                    if gc != ch {break;}
                    if ch == 'S' {count += 1}
                    cr += dr;
                    cc += dc;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);
    let mut count = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let row = row as isize;
            let col = col as isize;
            if grid.get(row, col) != Some(&'A') {continue;}
            let Some(&ul) = grid.get(row - 1, col - 1) else {continue;};
            let Some(&ur) = grid.get(row - 1, col + 1) else {continue;};
            let Some(&ll) = grid.get(row + 1, col - 1) else {continue;};
            let Some(&lr) = grid.get(row + 1, col + 1) else {continue;};
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
