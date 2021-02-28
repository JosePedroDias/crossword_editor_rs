extern crate ncurses;

use ncurses::*;
use std::char;
use std::env;

mod consts;
mod draw;
mod io;
mod types;

use consts::*;
use draw::*;
use io::*;
use types::*;

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
    let mut use_new_board = false;
    let mut w: usize = 11;
    let mut h: usize = 11;

    {
        let args: Vec<String> = env::args().collect();

        if args.len() >= 3 {
            w = args[1].parse::<usize>().unwrap_or(0);
            h = args[2].parse::<usize>().unwrap_or(0);
            if (2..30).contains(&w) && (2..20).contains(&h) {
                use_new_board = true;
            }
        }
    }

    let mut p = Pos { x: 0, y: 0 };
    let mut m = Matrix::new(w, h);
    let mut c: i32 = 0;
    let mut mode = Mode::HORIZONTAL;

    if use_new_board {
        println!("Setting up new board of {} x {}", m.width, m.height);
    } else {
        if let Ok(loaded) = load() {
            m = loaded;
            println!("Recovering board of {} x {}", m.width, m.height);
        } else {
            println!("Setting up new board of {} x {}", m.width, m.height);
        }
    }

    setup();

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
