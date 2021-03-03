extern crate ncurses;

use ncurses::*;

use crate::consts::*;
use crate::types::*;

pub fn setup() {
    setlocale(LcCategory::all, "pt_PT.UTF-8");

    initscr();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    start_color();

    //        pairNumber   foreground    background
    init_pair(CLR_GRID, COLOR_YELLOW, COLOR_BLACK);
    init_pair(CLR_FILLED, COLOR_BLACK, COLOR_WHITE);
    init_pair(CLR_CURSOR, COLOR_RED, COLOR_BLACK);
}

pub fn draw_grid(m: &Matrix) {
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

pub fn draw_status(p: &Pos, c: i32, mode: Mode) {
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

pub fn draw_cursor(p: &Pos) {
    attr_on(COLOR_PAIR(CLR_CURSOR));
    mvaddch((p.y * 2 + 1) as i32, (p.x * 2 + 1) as i32, '@' as u32);
    attr_off(COLOR_PAIR(CLR_CURSOR));
}

pub fn draw_cell(cell: &Cell, x: usize, y: usize) {
    let mut v = cell.value;
    if cell.filled {
        v = ' ';
    }

    if cell.filled {
        attr_on(A_REVERSE());
    }

    mvaddch((y * 2 + 1) as i32, (x * 2 + 1) as i32, v as u32);

    if cell.filled {
        attr_off(A_REVERSE());
    }
}

pub fn draw_cells(m: &Matrix) {
    let w = m.width;
    let h = m.height;
    for y in 0..h {
        for x in 0..w {
            let cell: &Cell = m.get_cell(x, y);
            draw_cell(&cell, x, y);
        }
    }
}
