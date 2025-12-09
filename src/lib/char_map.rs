// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn step_towards(&self, other: &Point) -> Self {
        if self.x > other.x {
            Point {
                x: self.x - 1,
                y: self.y,
            }
        } else if self.x < other.x {
            Point {
                x: self.x + 1,
                y: self.y,
            }
        } else if self.y < other.y {
            Point {
                x: self.x,
                y: self.y + 1,
            }
        } else if self.y > other.y {
            Point {
                x: self.x,
                y: self.y - 1,
            }
        } else {
            Point {
                x: self.x,
                y: self.y,
            }
        }
    }
}

pub struct Map {
    buffer: Vec<Vec<u8>>,
    pub height: usize,
    pub width: usize,
}

impl Map {
    pub fn new(buffer: Vec<Vec<u8>>) -> Self {
        Self {
            height: buffer.len(),
            width: buffer[0].len(),
            buffer,
        }
    }

    pub fn get(&self, p: Point) -> Option<u8> {
        self.buffer.get(p.y).and_then(|row| row.get(p.x).copied())
    }

    pub fn set(&mut self, p: Point, val: u8) {
        self.buffer[p.y][p.x] = val;
    }

    pub fn neighbours(&self, p: Point) -> Vec<Point> {
        let (x, y) = (p.x as i64, p.y as i64);
        [
            (x + 1, y),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x - 1, y - 1),
        ]
        .iter()
        .filter(|(x, y)| (0..self.width as i64).contains(x) && (0..self.height as i64).contains(y))
        .map(|(x, y)| Point {
            x: *x as usize,
            y: *y as usize,
        })
        .collect()
    }

    pub fn points(&self) -> impl Iterator<Item = (Point, u8)> {
        self.buffer.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, val)| (Point { x, y }, *val))
        })
    }
}
