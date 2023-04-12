#[derive(Debug)]
pub(crate) enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}
impl Dir {
    pub fn turn(&self) -> Self {
        match *self {
            Dir::UP => Dir::LEFT,
            Dir::LEFT => Dir::DOWN,
            Dir::DOWN => Dir::RIGHT,
            Dir::RIGHT => Dir::UP,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    pub fn next_cell(&self, dir: &Dir) -> Option<Coord> {
        match dir {
            Dir::UP => {
                return Some(Coord {
                    x: self.x,
                    y: self.y + 1,
                })
            }
            Dir::LEFT => {
                return Some(Coord {
                    x: self.x + 1,
                    y: self.y,
                })
            }
            Dir::DOWN => {
                if self.y == 0 {
                    return None;
                }
                return Some(Coord {
                    x: self.x,
                    y: self.y - 1,
                });
            }
            Dir::RIGHT => {
                if self.x == 0 {
                    return None;
                }
                return Some(Coord {
                    x: self.x - 1,
                    y: self.y,
                });
            }
        }
    }
}
