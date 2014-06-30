/*
 * Copyright (c) 2014 Dominator008
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use cpu::io::out;
use drivers::pic::remap;

/* Defines an IDT entry */
#[packed]
pub struct IDTEntry {
  base_lo: u16,
  sel: u16,        /* Our kernel segment goes here! */
  zero_0: u8,      /* This will ALWAYS be set to 0! */
  flags: u8,       /* Set using the above table! */
  base_mid: u16,
  base_hi: u32,
  zero_1: u32
}

/* Defines an IDT pointer */
#[packed]
pub struct IDTPointer {
  limit: u16,
  base: u64
}

/**
 * Declare an IDT of 256 entries. If any undefined IDT entry is hit,
 * it normally will cause an "Unhandled Interrupt" exception. Any
 * descriptor for which the 'presence' bit is cleared (0) will generate
 * an "Unhandled Interrupt" exception.
 */
#[no_mangle]
pub static mut idt: [IDTEntry, ..256] = [IDTEntry {
    base_lo: 0, sel: 0, zero_0: 0, flags: 0, base_mid: 0, base_hi: 0, zero_1: 0
}, ..256];

#[no_mangle]
pub static mut idtp: IDTPointer = IDTPointer {limit: 0, base: 0};

/**
 * Use this function to set an entry in the IDT. A lot simpler
 * than twiddling with the GDT ;)
 */
#[no_mangle]
fn idt_set_gate(num: u8, f: unsafe extern "C" fn(), sel: u16, flags: u8) {
  unsafe {
    let base = f as u64;
        idt[num as uint].sel = sel;
        idt[num as uint].flags = flags;
        idt[num as uint].base_hi = (base >> 32) as u32;
        idt[num as uint].base_mid = ((base >> 16) & ((1 << 16) - 1)) as u16;
        idt[num as uint].base_lo = (base & ((1 << 16) - 1)) as u16;
  }
}

/* Installs the IDT */
extern {
  fn int_handler_kbd_wrapper();
}

#[no_mangle]
pub unsafe fn install_idt() {
  /* Sets the special IDT pointer up  */
  idtp.limit = ((super::super::core::mem::size_of::<IDTEntry>() * 256) - 1) as u16;
  idtp.base = &idt as *[IDTEntry, ..256] as u64;

  /* Add any new ISRs to the IDT here using idt_set_gate */
  idt_set_gate(33, int_handler_kbd_wrapper, 0x08, 0x8E);

  /* Remap the PIC */
  remap();

  out(0x21, 0xfd); // Keyboard interrupts only
  out(0xa1, 0xff);

  /* Turn interrupts on */
  asm!("lidtq ($0) \n
        sti"
       :: "r" (&idtp)
      );
}
