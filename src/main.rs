extern crate ncurses;
extern crate serde;
extern crate serde_json;

use ncurses::*;
use serde::{Deserialize, Serialize};
use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

static UP: i32 = 65;
static DOWN: i32 = 66;
static LEFT: i32 = 68;
static RIGHT: i32 = 67;
//static ESCAPE: i32 = 27;
static Q: i32 = 81;
static TAB: i32 = 9;
static ENTER: i32 = 10;
static BCKSPC: i32 = 127;
static C_A: i32 = 97;
static C_Z: i32 = 122;
static SPACE: i32 = 32;

static CLR_GRID: i16 = 1;
static CLR_FILLED: i16 = 2;
static CLR_CURSOR: i16 = 3;

static STATUS_Y: i32 = 25;

static FILENAME: &str = "game.json";

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    HORIZONTAL,
    VERTICAL,
}

struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Cell {
    filled: bool,
    value: char,
}

#[derive(Debug, Serialize, Deserialize)]
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

    /* fn get_value(&self, x: usize, y: usize) -> char {
        let i = self.get_index(x, y);
        self.cells[i].value
    } */

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

fn load() -> Option<Matrix> {
    // TODO proper error handling
    let mut input = File::open(FILENAME).unwrap();
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let matrix: Matrix = serde_json::from_str(&str).unwrap();
    //println!("loaded: {:?}", matrix);

    Some(matrix)
}

fn save(m: &Matrix) {
    // TODO proper error handling
    // -> Result<()> {
    let serialized = serde_json::to_string(&m).unwrap();
    //println!("saved: {}", serialized);

    let mut output = File::create(FILENAME).unwrap();
    write!(output, "{}", serialized).unwrap();

    //Ok(())
}

fn draw_grid(m: &Matrix) {
    attr_on(COLOR_PAIR(CLR_GRID));

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

    attr_off(COLOR_PAIR(CLR_GRID));
}

fn draw_status(p: &Pos, c: i32, mode: Mode) {
    let mode_s: &str = if mode == Mode::HORIZONTAL {
        "HOR"
    } else {
        "VER"
    };

    mvprintw(
        STATUS_Y,
        0,
        &format!("pos: {},{} | mode: {} | char: {}   ", p.x, p.y, mode_s, c)[..],
    );
}

fn draw_cursor(p: &Pos) {
    attr_on(COLOR_PAIR(CLR_CURSOR));
    mvaddch((p.y * 2 + 1) as i32, (p.x * 2 + 1) as i32, '@' as u64);
    attr_off(COLOR_PAIR(CLR_CURSOR));
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
        if mode == Mode::HORIZONTAL {
            p.x = (p.x as i32 + delta) as usize;
        } else {
            p.y = (p.y as i32 + delta) as usize;
        }
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
    let mut m = Matrix::new(11, 11);
    let mut c: i32 = 0;
    let mut mode = Mode::HORIZONTAL;

    //m.set_filled(1, 0, true);
    //m.set_value(0, 2, 'X');

    let _m: Option<Matrix> = load();
    if _m.is_some() {
        m = _m.unwrap();
    }

    setlocale(LcCategory::all, "pt_PT.UTF-8");

    initscr();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    //keypad(stdscr(), true);
    noecho();
    start_color();

    //        pairNumber   foreground    background
    init_pair(CLR_GRID, COLOR_YELLOW, COLOR_BLACK);
    init_pair(CLR_FILLED, COLOR_BLACK, COLOR_WHITE);
    init_pair(CLR_CURSOR, COLOR_RED, COLOR_BLACK);

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

    save(&m);

    endwin();
}
