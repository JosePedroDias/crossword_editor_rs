#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    HORIZONTAL,
    VERTICAL,
}

pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Cell {
    pub filled: bool,
    pub value: char,
}

impl Cell {
    pub fn toggle_filled(&mut self) {
        self.filled = !self.filled;
        if self.filled {
            self.value = ' ';
        }
    }
}

pub struct Matrix {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            cells: vec![
                Cell {
                    filled: false,
                    value: ' '
                };
                width * height
            ],
        }
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let i = y * self.width + x;
        &mut self.cells[i]
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        let i = y * self.width + x;
        &self.cells[i]
    }
}
