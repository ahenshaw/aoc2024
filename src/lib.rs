use std::collections::HashMap;

pub mod template;

pub struct Grid<T> {
    container: HashMap<(isize, isize), T>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid<char> {
    pub fn from_str(input: &str) -> Self {
        let mut container = HashMap::<(isize, isize), char>::new();

        let rows = input.lines().count();
        let cols = input.len() / rows;
        for (col, line) in input.lines().enumerate() {
            for (row, ch) in line.chars().enumerate() {
                container.insert((row as isize, col as isize), ch);
            }
        }
        Self { container, rows, cols }
    }
}

impl <T> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&(y, x))
    }
}
