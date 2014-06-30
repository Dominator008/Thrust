/*
 * Copyright (c) 2014 Dominator008
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::fmt;
use core::prelude::*;

use super::drivers::vga;

struct Stdout;

impl Stdout {
  fn write_fmt(&mut self, fmt: &fmt::Arguments) {
    fmt::write(self, fmt);
  }
}

impl fmt::FormatWriter for Stdout {
  fn write(&mut self, bytes: &[u8]) -> fmt::Result {
    for &c in bytes.iter() {
      unsafe {
        putc(c);
      }
    }
    Ok(())
  }
}

pub fn print_args(fmt: &fmt::Arguments) {
  write!(Stdout, "{}", fmt);
}

pub fn println_args(fmt: &fmt::Arguments) {
  writeln!(Stdout, "{}", fmt);
}

pub unsafe fn putc(c: u8) {
  vga::print_byte_default(c);
}

pub unsafe fn puts(s: &str) {
  for c in s.bytes() {
    putc(c);
  }
}
