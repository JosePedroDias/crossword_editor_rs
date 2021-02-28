use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::str;

use crate::consts::*;
use crate::types::*;

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

pub fn load() -> Result<Matrix, std::io::Error> {
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

pub fn save(m: &Matrix) -> Result<(), std::io::Error> {
    let mut output_file = File::create(FILENAME)?;
    write!(output_file, "{}\n", FILE_FORMAT_VERSION)?;
    write!(output_file, "{},{}\n", m.width, m.height)?;

    let w = m.width;
    let h = m.height;
    for y in 0..h {
        for x in 0..w {
            let cell = m.get_cell(x, y);
            write!(output_file, "{},{}\n", cell.filled, cell.value)?;
        }
    }

    Ok(())
}
