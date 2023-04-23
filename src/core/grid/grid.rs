use std::fmt::Display;

use super::{Cell, Coord, Export, Team};
#[derive(Clone)]
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
    pub fn export(&self, frames: usize, teams: Vec<Team>) -> Export {
        let data = self
            .grid
            .iter()
            .map(|r| r.iter().map(|c| c.color()).collect())
            .collect();
        return Export::new(data, self.rows, self.cols, frames, teams);
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
