use strum_macros::EnumIter;
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
pub(crate) struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    pub fn distance(&self, other: &Coord) -> f64 {
        return (((other.x.abs_diff(self.x)).pow(2) + (other.y.abs_diff(self.y)).pow(2)) as f64)
            .sqrt();
    }
    pub fn next_cell(&self, dir: &Dir) -> Option<Coord> {
        return match dir {
            Dir::UP => Some(Coord {
                x: self.x,
                y: self.y + 1,
            }),
            Dir::LEFT => Some(Coord {
                x: self.x + 1,
                y: self.y,
            }),
            Dir::DOWN => {
                if self.y == 0 {
                    return None;
                }
                Some(Coord {
                    x: self.x,
                    y: self.y - 1,
                })
            }
            Dir::RIGHT => {
                if self.x == 0 {
                    return None;
                }
                Some(Coord {
                    x: self.x - 1,
                    y: self.y,
                })
            }
        };
    }
}
