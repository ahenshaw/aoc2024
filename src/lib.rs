use std::collections::HashMap;

pub mod template;
#[derive(Hash, Debug, Clone, Eq, PartialEq, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub container: HashMap<Coord, T>,
    pub height: usize,
    pub width: usize,
}

impl Grid<char> {
    pub fn from_str(input: &str) -> Self {
        let mut container = HashMap::<Coord, char>::new();

        let height = input.lines().count();
        let width = input.len() / height;
        for (col, line) in input.lines().enumerate() {
            for (row, ch) in line.chars().enumerate() {
                container.insert(
                    Coord {
                        x: col as isize,
                        y: row as isize,
                    },
                    ch,
                );
            }
        }
        Self {
            container,
            height,
            width,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y })
    }
    pub fn north(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y: y - 1 })
    }
    pub fn south(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y: y + 1 })
    }
    pub fn east(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x + 1, y })
    }
    pub fn west(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x - 1, y })
    }
    pub fn northwest(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x - 1, y: y - 1 })
    }
    pub fn northeast(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x + 1, y: y - 1 })
    }
    pub fn southwest(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x - 1, y: y + 1 })
    }
    pub fn southeast(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x + 1, y: y + 1 })
    }

    pub fn is_in(&self, x: isize, y: isize) -> bool {
        (0..self.width as isize).contains(&x)
        && (0..self.height as isize).contains(&y)
    }
}

pub const NEIGHBORS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (-1, -1),
    (-1, 1),
    (-1, 0),
];

impl FromIterator<char> for Grid<char> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut container = HashMap::<Coord, char>::new();
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut height = 0;
        let mut width = 0;
        for ch in iter {
            match ch {
                '\r' => (),
                '\n' => {
                    y += 1;
                    x = 0;
                }
                _ => {
                    container.insert(Coord { x, y }, ch);
                    height = height.max(y as usize + 1);
                    width = width.max(x as usize + 1);
                    x += 1;
                }
            }
        }

        Grid {
            container,
            height: height,
            width: width,
        }
    }
}
