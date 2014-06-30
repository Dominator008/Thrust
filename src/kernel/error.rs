/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::drivers::vga;

pub fn panic(reason: &str) -> ! {
  vga::color_println("Ohs noes!  You've been harpooned!", vga::Red, vga::BACKGROUND_COLOR);
  // print fail whale (kraken?)
  vga::print("Reason: ");
  vga::print(reason);
  // wait 10 seconds
  abort();
}

#[no_mangle]
#[inline(always)]
pub fn abort() -> ! {
  unsafe { super::reset::immediate_reset(); }
}

#[no_mangle]
pub fn rust_begin_unwind(_: &::core::fmt::Arguments, file: &'static str, _: uint) -> ! {
  panic(file);
}
