/*
 * Copyright (c) 2014 Dominator008
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::mem::transmute;

use cpu::io::out;
//use platform::runtime::wmemset;

use core::iter;
use core::iter::Iterator;
use core::option::{Some, None};
use core::str::StrSlice;

#[repr(u8)]
pub enum Color {
  Black       = 0,
  Blue        = 1,
  Green       = 2,
  Cyan        = 3,
  Red         = 4,
  Pink        = 5,
  Brown       = 6,
  LightGray   = 7,
  DarkGray    = 8,
  LightBlue   = 9,
  LightGreen  = 10,
  LightCyan   = 11,
  LightRed    = 12,
  LightPink   = 13,
  Yellow      = 14,
  White       = 15,
}

#[packed]
struct ScreenChar {
  pub char: u8,
  attr: u8,
}

impl ScreenChar {
  #[inline]
  pub fn new(c: char, fg: Color, bg: Color) -> ScreenChar {
    ScreenChar { char: c as u8, attr: fg as u8 | (bg as u8 << 4) }
  }
}

pub static MAX_ROW: uint = 25;
pub static MAX_COLUMN: uint = 80;
pub static SCREEN_ADDR: uint = 0xb8000;

pub static SCREEN_SIZE: uint = MAX_ROW * MAX_COLUMN;
pub type Screen = [ScreenChar, ..SCREEN_SIZE];
pub static SCREEN: *mut Screen = SCREEN_ADDR as *mut Screen;

pub static BACKGROUND_COLOR: Color = Black;
pub static FOREGROUND_COLOR: Color = Green;

static mut row: uint = 0;
static mut col: uint = 0;

/*pub unsafe fn clear_screen(bg: Color) {
  wmemset(SCREEN as *mut u8, transmute(ScreenChar::new(' ', BACKGROUND_COLOR, bg)), SCREEN_SIZE);
}*/

pub unsafe fn move_cursor_pos(pos: uint) {
  out(0x3D4, 15 as u8);
  out(0x3D5, pos as u8);
  out(0x3D4, 14 as u8);
  out(0x3D5, (pos >> 8) as u8);
}

pub fn move_cursor(row: uint, col: uint) {
  unsafe {
    move_cursor_pos(row * MAX_COLUMN + col);
  }
}

#[inline]
pub fn print(msg: &str) {
  color_print(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

#[inline]
pub fn println(msg: &str) {
  color_println(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

#[inline]
pub fn clear_screen() {
  color_clear_screen(BACKGROUND_COLOR);
}

pub fn color_println(msg: &str, foreground: Color, background: Color) {
  color_print(msg, foreground, background);
  print_byte('\n' as u8, foreground, background);
  unsafe {
    move_cursor(row, col);
  }
}

pub fn color_print(msg: &str, foreground: Color, background: Color) {
  unsafe {
    for byte in msg.bytes() {
      print_byte(byte, foreground, background);
    }
    move_cursor(row, col);
  }
}

#[inline]
pub unsafe fn print_bytes(msg: *const u8) {
  color_print_bytes(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

pub unsafe fn color_print_bytes(msg: *const u8, foreground: Color, background: Color) {
  let mut msg = msg;
  while *msg != 0 {
    print_byte(*msg, foreground, background);
    msg = (msg as uint + 1) as *const u8;
  }
  move_cursor(row, col);
}

#[inline]
fn print_byte(byte: u8, foreground: Color, background: Color) {
  unsafe {
    match byte {
      0x0a /* newline */ => add_line(background),
      0x0d /* carriage return */ => col = 0,
      0x08 /* backspace */ => {
        if col == 0 && row != 0 {
          col = MAX_COLUMN - 1;
          row -= 1;
        } else if col != 0 {
          col -= 1;
        }
        let pos = row * MAX_COLUMN + col;
        (*SCREEN)[pos].char = 0 as u8;
        (*SCREEN)[pos].attr = ((background as u8) << 4) + (foreground as u8);
      }
      byte => {
        let pos = row * MAX_COLUMN + col;
        (*SCREEN)[pos].char = byte as u8;
        (*SCREEN)[pos].attr = ((background as u8) << 4) + (foreground as u8);
        col += 1;
        if col == MAX_COLUMN {
          add_line(background);
        }
      }
    }
    move_cursor(row, col);
  }
}

#[inline]
pub fn print_byte_default(byte: u8) {
  print_byte(byte, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

pub fn color_clear_screen(background: Color) {
  unsafe {
    for line in iter::range(0, MAX_ROW) {
      clear_line(line, background);
    }
    row = 0;
    col = 0;
    move_cursor(0, 0);
  }
}

fn clear_line(_row: uint, background: Color) {
  unsafe {
    let c = col;
    let r = row;
    col = 0;
    row = _row;
    clear_rem_line(background);
    row = r;
    col = c;
  }
}

fn clear_rem_line(background: Color) {
  unsafe {
    let rpos = row * MAX_COLUMN;
    for i in iter::range(col, MAX_COLUMN) {
      let pos = rpos + i;
      (*SCREEN)[pos].char = ' ' as u8;
      (*SCREEN)[pos].attr = ((background as u8) << 4) + (FOREGROUND_COLOR as u8);
    }
  }
}

fn add_line(background: Color) {
  clear_rem_line(background);
  unsafe {
    col = 0;
    row += 1;
    if row == MAX_ROW {
      row -= 1;
      shift_rows_up();
    }
    move_cursor(row, col);
  }
}

fn shift_rows_up() {
  unsafe {
    for r in iter::range(1, MAX_ROW) {
      let fposr = r * MAX_COLUMN;
      let tposr = fposr - MAX_COLUMN;
      for c in iter::range(0, MAX_COLUMN) {
        let tpos = tposr + c;
        let fpos = fposr + r;
        (*SCREEN)[tpos].char = (*SCREEN)[fpos].char;
        (*SCREEN)[tpos].attr = (*SCREEN)[fpos].attr;
      }
    }
  }
  clear_line(MAX_ROW - 1, BACKGROUND_COLOR);
}

pub fn backspace() {
  unsafe {
    if col == 0 {
      row -= 1;
      col = MAX_COLUMN - 1;
    } else {
      col -= 1;
    }
    move_cursor(row, col);
    print_byte_default(0);
  }
}

pub fn newline() {
  add_line(BACKGROUND_COLOR);
}
