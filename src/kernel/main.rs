/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![crate_id(name = "kRnel",
            vers = "0.0.1",
            author = "Arcterus",
            license = "MPL v2.0")]

#![allow(ctypes)]
#![no_std]
#![feature(globs)]
#![feature(asm)]

extern crate core;
extern crate rlibc;

pub use core::prelude::*;

pub use drivers::io::console;
pub use platform::*;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod platform;

#[path = "../drivers"]
pub mod drivers {
  pub mod io {
    pub mod console;
  }
}

pub mod kbd;
pub mod idt;
pub mod gdt;
pub mod stdio;
pub mod memory;
pub mod error;
pub mod support;
pub mod pic;
pub mod io;

#[no_mangle]
pub fn main(mem: *memory::BootMemMap) {
  let mem: &memory::BootMemMap = unsafe { &(*mem) };
  console::clear_screen();
  console::print("iiiiiiiiiiiiiiiiiiiiiiiiiii\niiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\x08\x08\x08\x08\x08test");
  console::println("");

  unsafe {
    //gdt::install_gdt();
    idt::install_idt();
  }

  let usable = mem.usable();
  let mut len = usable.len();
  while len > 0 {
    unsafe { console::print_bytes([(len % 10 + '0' as uint) as u8, 0].as_ptr()); }
    len /= 10;
    console::println("");
  }
  loop {}
}
