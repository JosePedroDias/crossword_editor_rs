extern crate ncurses;

use ncurses::*;

#[derive(Clone)]
struct Cell {
    filled: bool,
    value: char,
}

struct Matrix {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Matrix {
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

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_value(&mut self, x: usize, y: usize) -> char {
        let i = self.get_index(x, y);
        self.cells[i].value
    }

    fn get_filled(&mut self, x: usize, y: usize) -> bool {
        let i = self.get_index(x, y);
        self.cells[i].filled
    }

    fn set_value(&mut self, x: usize, y: usize, v: char) {
        let i = self.get_index(x, y);
        self.cells[i].value = v;
    }

    fn set_filled(&mut self, x: usize, y: usize, f: bool) {
        let i = self.get_index(x, y);
        self.cells[i].filled = f;
    }
}

fn main() {
    let mut m = Matrix::new(4, 3);
    m.set_filled(1, 0, true);
    m.set_value(0, 2, 'X');
    //m[2].filled = true;
    //m[2].value = 'x';

    setlocale(LcCategory::all, "pt_PT.UTF-8");

    initscr();

    mvaddch(3, 3, '@' as u64);
    addstr("Hello, world!");

    refresh();

    getch();

    endwin();
}
