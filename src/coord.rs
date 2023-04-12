#[derive(Debug)]
pub(crate) enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    pub fn next_cell(&self, dir: &Dir) -> Coord {
        match dir {
            Dir::UP => {
                return Coord {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            Dir::LEFT => {
                return Coord {
                    x: self.x + 1,
                    y: self.y,
                }
            }
            Dir::DOWN => {
                return Coord {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            Dir::RIGHT => {
                return Coord {
                    x: self.x - 1,
                    y: self.y,
                }
            }
        }
    }
}
