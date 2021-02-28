extern crate ncurses;

use ncurses::*;
use std::char;
//use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::str;

const FILE_FORMAT_VERSION: u8 = 1;

const CH_COMMA: char = ',';
const CH_NL: char = '\n';

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

const CLR_GRID: i16 = 1;
const CLR_FILLED: i16 = 2;
const CLR_CURSOR: i16 = 3;

const STATUS_Y: i32 = 25;

const FILENAME: &str = "game.txt";

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

impl Cell {
    fn toggle_filled(&mut self) {
        self.filled = !self.filled;
        if self.filled {
            self.value = ' ';
        }
    }
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

    fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let i = y * self.width + x;
        &mut self.cells[i]
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        let i = y * self.width + x;
        &self.cells[i]
    }
}

fn read_str_until_char(reader: &mut BufReader<File>, ch: char) -> String {
    let mut arr = Vec::<u8>::new();
    reader.read_until(ch as u8, &mut arr).unwrap_or(0);
    let s = str::from_utf8(&arr).unwrap();
    return (&s[0..s.len() - 1]).to_string();
}

fn read_char(reader: &mut BufReader<File>) -> char {
    let mut arr = [0u8; 1];
    let mut ad = reader.take(1);
    ad.read(&mut arr).unwrap_or(0);
    return arr[0] as char;
}

fn string_to_i32(s: &String) -> i32 {
    return s.parse::<i32>().unwrap_or(0);
}

fn string_to_bool(s: &String) -> bool {
    return s.parse::<bool>().unwrap_or(false);
}

fn string_to_char(s: &String) -> char {
    return s.parse::<char>().unwrap_or(' ');
}

fn load() -> Result<Matrix, std::io::Error> {
    let input_file = File::open(FILENAME)?;
    let mut buf_reader = BufReader::new(input_file);

    let file_format_version = read_char(&mut buf_reader) as u8;
    read_char(&mut buf_reader);

    if (file_format_version - '0' as u8) != FILE_FORMAT_VERSION {
        panic!("Unsupported format");
    }

    let w = string_to_i32(&read_str_until_char(&mut buf_reader, CH_COMMA)) as usize;
    let h = string_to_i32(&read_str_until_char(&mut buf_reader, CH_NL)) as usize;

    let mut m = Matrix::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let filled = string_to_bool(&read_str_until_char(&mut buf_reader, CH_COMMA));
            let value = string_to_char(&read_str_until_char(&mut buf_reader, CH_NL));

            let cell = m.get_cell_mut(x, y);
            cell.filled = filled;
            cell.value = value;
        }
    }

    Ok(m)
}

fn save(m: &Matrix) -> Result<(), std::io::Error> {
    let mut output_file = File::create(FILENAME)?;
    write!(output_file, "{}\n", FILE_FORMAT_VERSION);
    write!(output_file, "{},{}\n", m.width, m.height);

    let w = m.width;
    let h = m.height;
    for y in 0..h {
        for x in 0..w {
            let cell = m.get_cell(x, y);
            write!(output_file, "{},{}\n", cell.filled, cell.value);
        }
    }

    Ok(())
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
            let cell: &Cell = m.get_cell(x, y);
            draw_cell(&cell, x, y);
        }
    }
}

fn advance(m: &Matrix, mode: Mode, p: &mut Pos, width: usize, height: usize, delta: i32) {
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
        let cell = m.get_cell(p.x, p.y);
        draw_cell(&cell, p.x, p.y);
        if mode == Mode::HORIZONTAL {
            p.x = (p.x as i32 + delta) as usize;
        } else {
            p.y = (p.y as i32 + delta) as usize;
        }
    }
}

fn process_input(c: i32, m: &mut Matrix, p: &mut Pos, mode: Mode) -> (bool, Mode) {
    let mut mode = mode;

    match c {
        LEFT if p.x > 0 => p.x -= 1,
        RIGHT if p.x < m.width - 1 => p.x += 1,
        UP if p.y > 0 => p.y -= 1,
        DOWN if p.y < m.height - 1 => p.y += 1,
        ENTER => {
            let cell = m.get_cell_mut(p.x, p.y);
            cell.toggle_filled();
            advance(&m, mode, p, m.width, m.height, 1);
        }
        SPACE | C_A..=C_Z => {
            let mut cell = m.get_cell_mut(p.x, p.y);
            if !cell.filled {
                cell.value = char::from_u32(c as u32).unwrap();
            }
            advance(&m, mode, p, m.width, m.height, 1);
        }
        BCKSPC => {
            let mut cell = m.get_cell_mut(p.x, p.y);
            cell.value = ' ';
            advance(&m, mode, p, m.width, m.height, -1);
        }
        TAB => {
            mode = if mode == Mode::HORIZONTAL {
                Mode::VERTICAL
            } else {
                Mode::HORIZONTAL
            };
        }
        Q => return (false, mode),
        _ => {}
    }

    (true, mode)
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    let mut p = Pos { x: 0, y: 0 };
    let mut m = Matrix::new(11, 11);
    let mut c: i32 = 0;
    let mut mode = Mode::HORIZONTAL;

    if let Ok(loaded) = load() {
        m = loaded;
    }

    setlocale(LcCategory::all, "pt_PT.UTF-8");

    initscr();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
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

    save(&m).ok();

    endwin();
}
