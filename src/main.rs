extern crate ncurses;

use ncurses::*;
use std::char;

const UP: i32 = 65;
const DOWN: i32 = 66;
const LEFT: i32 = 68;
const RIGHT: i32 = 67;
//const ESCAPE: i32 = 27;
const Q: i32 = 81;
const TAB: i32 = 9;
const ENTER: i32 = 10;
const BCKSPC: i32 = 127;
const C_A: i32 = 97;
const C_Z: i32 = 122;
const SPACE: i32 = 32;

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    HORIZONTAL,
    VERTICAL,
}

struct Pos {
    x: usize,
    y: usize,
}

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

    fn get_value(&self, x: usize, y: usize) -> char {
        let i = self.get_index(x, y);
        self.cells[i].value
    }

    fn get_filled(&self, x: usize, y: usize) -> bool {
        let i = self.get_index(x, y);
        self.cells[i].filled
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        let i = self.get_index(x, y);
        &self.cells[i]
    }

    fn toggle_filled(&mut self, x: usize, y: usize) {
        let i = self.get_index(x, y);
        let c: &mut Cell = &mut self.cells[i];
        c.filled = !c.filled;
        if c.filled {
            c.value = ' ';
        }
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

fn load() {}

fn save() {}

fn draw_grid(m: &Matrix) {
    attr_on(A_BOLD());

    let w = m.width as i32;
    let h = m.height as i32;
    for y in 0..=h {
        for x in 0..=w {
            if x == w {
                mvprintw(y * 2, w * 2, "+");
            } else {
                mvprintw(y * 2, x * 2, "+-");
            }
            if y != h {
                mvprintw(y * 2 + 1, x * 2, "|");
            }
        }
    }

    attr_off(A_BOLD());
}

fn draw_status(p: &Pos, c: i32, mode: Mode) {
    let mode_s: &str = if mode == Mode::HORIZONTAL {
        "HOR"
    } else {
        "VER"
    };

    mvprintw(
        30,
        0,
        &format!("pos: {},{} | mode: {} | char: {}   ", p.x, p.y, mode_s, c)[..],
    );
}

fn draw_cursor(p: &Pos) {
    //attr_on(COLOR_PAIR(CLR_CURSOR));
    mvaddch((p.y * 2 + 1) as i32, (p.x * 2 + 1) as i32, '@' as u64);
    //attr_off(COLOR_PAIR(CLR_CURSOR));
}

fn draw_cell(cell: &Cell, x: usize, y: usize) {
    let mut v = cell.value;
    if cell.filled {
        v = ' ';
    }

    if cell.filled {
        attr_on(A_REVERSE());
    }

    mvaddch((y * 2 + 1) as i32, (x * 2 + 1) as i32, v as u64);

    if cell.filled {
        attr_off(A_REVERSE());
    }
}

fn draw_cells(m: &Matrix) {
    let w = m.width;
    let h = m.height;
    for y in 0..h {
        for x in 0..w {
            draw_cell(m.get_cell(x, y), x, y);
        }
    }
}

fn advance(cell: &Cell, mode: Mode, p: &mut Pos, width: usize, height: usize, delta: i32) {
    let coord: usize = if mode == Mode::HORIZONTAL { p.x } else { p.y };

    let max_value = if mode == Mode::HORIZONTAL {
        width
    } else {
        height
    };
    let valid = if delta == 1 {
        coord < max_value - 1
    } else {
        coord > 0
    };

    if valid {
        draw_cell(cell, p.x, p.y);
    }
}

fn process_input(c: i32, m: &mut Matrix, p: &mut Pos, mode_: Mode) -> (bool, Mode) {
    let mut mode = mode_;

    if c == Q {
        return (false, mode);
    } else if c == TAB {
        mode = if mode == Mode::HORIZONTAL {
            Mode::VERTICAL
        } else {
            Mode::HORIZONTAL
        };
    } else if c == LEFT && p.x > 0 {
        p.x -= 1;
    } else if c == RIGHT && p.x < m.width - 1 {
        p.x += 1;
    } else if c == UP && p.y > 0 {
        p.y -= 1;
    } else if c == DOWN && p.y < m.height - 1 {
        p.y += 1;
    } else if c == ENTER {
        m.toggle_filled(p.x, p.y);
        let cell: &Cell = m.get_cell(p.x, p.y);
        advance(&cell, mode, p, m.width, m.height, 1);
    } else if c >= C_A && c <= C_Z || c == SPACE {
        if !m.get_filled(p.x, p.y) {
            m.set_value(p.x, p.y, char::from_u32(c as u32).unwrap());
            let cell: &Cell = m.get_cell(p.x, p.y);
            advance(&cell, mode, p, m.width, m.height, 1);
        }
    } else if c == BCKSPC {
        m.set_value(p.x, p.y, ' ');
        let cell: &Cell = m.get_cell(p.x, p.y);
        advance(&cell, mode, p, m.width, m.height, -1);
    }
    return (true, mode);
}

fn main() {
    let mut p = Pos { x: 0, y: 0 };
    let mut m = Matrix::new(4, 3);
    let mut c: i32 = 0;
    let mut mode = Mode::HORIZONTAL;

    m.set_filled(1, 0, true);
    m.set_value(0, 2, 'X');

    setlocale(LcCategory::all, "pt_PT.UTF-8");

    initscr();

    noecho();

    draw_grid(&m);
    draw_cells(&m);

    loop {
        draw_cursor(&p);
        draw_status(&p, c, mode);
        refresh();
        c = getch();

        draw_cell(m.get_cell(p.x, p.y), p.x, p.y);

        let (cont, mode_) = process_input(c, &mut m, &mut p, mode);
        mode = mode_;
        if !cont {
            break;
        }
    }

    //save();

    endwin();
}
