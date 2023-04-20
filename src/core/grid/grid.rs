use std::fmt::Display;

use super::{Cell, Coord};

pub(crate) struct Grid {
    pub grid: Vec<Vec<Cell>>,
    pub rows: usize,
    pub cols: usize,
}
impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Vec::with_capacity(cols);
        for x in 0..cols {
            grid.push(Vec::with_capacity(rows));
            for _ in 0..rows {
                grid.get_mut(x).unwrap().push(Cell::default());
            }
        }
        return Grid { grid, rows, cols };
    }
    pub fn get(&self, c: &Coord) -> &Cell {
        assert!(self.does_exist(c));
        return self.grid.get(c.x).unwrap().get(c.y).unwrap();
    }
    pub fn get_mut(&mut self, c: &Coord) -> &mut Cell {
        assert!(self.does_exist(c));
        return self.grid.get_mut(c.x).unwrap().get_mut(c.y).unwrap();
    }
    pub fn does_exist(&self, coord: &Coord) -> bool {
        if coord.x >= self.cols || coord.y >= self.rows {
            return false;
        }
        return true;
    }
    pub fn iter(&self) -> GridIterator {
        return GridIterator {
            grid: self,
            coord: Coord { x: 0, y: 0 },
        };
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            let y = self.rows - i - 1;
            write!(f, "|")?;
            for x in 0..self.cols {
                write!(f, " ")?;
                write!(f, "{}", self.get(&Coord { x, y }))?;
                write!(f, " |")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
pub(crate) struct GridIterator<'a> {
    grid: &'a Grid,
    coord: Coord,
}
impl<'a> Iterator for GridIterator<'a> {
    type Item = (Coord, &'a Cell);
    fn next(&mut self) -> Option<Self::Item> {
        if !self.grid.does_exist(&self.coord) {
            return None;
        }
        let c = self.coord.clone();
        let res = self.grid.get(&self.coord);
        if self.coord.x + 1 < self.grid.cols {
            self.coord.x += 1;
        } else {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        return Some((c, res));
    }
}
