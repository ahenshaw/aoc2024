use std::collections::HashMap;
use std::ops::Add;

pub mod template;
#[derive(Hash, Debug, Clone, Eq, PartialEq, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Add<Direction> for Coord {
    type Output = Coord;
    fn add(self, dir: Direction) -> Coord {
        let (dx, dy) = dir.to_delta();
        Coord {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}
pub const CARDINAL: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl Direction {
    fn to_delta(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::Northeast => (1, -1),
            Self::Northwest => (-1, -1),
            Self::Southeast => (1, 1),
            Self::Southwest => (-1, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub container: HashMap<Coord, T>,
    pub height: usize,
    pub width: usize,
}

pub struct GridIter<'a> {
    current: Coord,
    grid: &'a Grid<char>,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (Coord, char);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(val) = self.grid.get(&self.current) else {
            self.current = Coord { x: 0, y: 0 };
            return None;
        };
        let item = Some((self.current.clone(), val.clone()));
        self.current.x += 1;
        if self.current.x == self.grid.width as isize {
            self.current.x = 0;
            self.current.y += 1;
        }
        item
    }
}

impl Grid<char> {
    pub fn iter(&self) -> GridIter<'_> {
        GridIter {
            current: Coord { x: 0, y: 0 },
            grid: self,
        }
    }
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
    pub fn get_xy(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y })
    }
    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.container.get(coord)
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
        (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y)
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
