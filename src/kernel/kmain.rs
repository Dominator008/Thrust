/*
 * Copyright (c) 2014 Dominator008
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![crate_id(name = "Thrust",
            vers = "0.0.1",
            author = "Dominator008",
            license = "MPL v2.0")]

#![allow(ctypes)]
#![no_std]
#![feature(asm, macro_rules, default_type_params, phase, globs)]

#[phase(plugin, link)]
extern crate core;
extern crate rlibc;

pub use core::prelude::*;
pub use core::fmt;

pub use platform::*;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod platform;

#[path = "../drivers"]
pub mod drivers {
  pub mod keyboard;
  pub mod pic;
  pub mod vga;
}

#[path = "./cpu"]
pub mod cpu {
  pub mod idt;
  pub mod io;
  pub mod gdt;
}

pub mod macros;
pub mod stdio;
pub mod memory;
pub mod error;
pub mod support;

mod std {
  // macros refer to absolute paths
  pub use core::fmt;
  pub use core::option;
}

#[no_mangle]
pub fn kmain(mem: *const memory::BootMemMap) {
  let mem: &memory::BootMemMap = unsafe { &(*mem) };
  drivers::vga::clear_screen();
  println!("Thrust full throttle, version {}", "0.0.1");

  unsafe {
    //cpu::gdt::install_gdt();
    cpu::idt::install_idt();
  }

  let usable = mem.usable();
  let mut len = usable.len();
  while len > 0 {
    unsafe { drivers::vga::print_bytes([(len % 10 + '0' as uint) as u8, 0].as_ptr()); }
    len /= 10;
  }
  loop {}
}
