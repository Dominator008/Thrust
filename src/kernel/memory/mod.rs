/*
 * Copyright (c) 2014 Dominator008
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::mem::size_of;
use core::option::{Option, Some, None};

pub use self::platform::*;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
mod platform;

pub struct MemoryMap<'a> {
  entries: *mut &'a Entry,
  length: uint
}

impl<'a> MemoryMap<'a> {
  pub fn new(entries: *mut &'a Entry, length: uint) -> MemoryMap<'a> {
    MemoryMap {
      entries: entries,
      length: length
    }
  }

  pub fn len(&self) -> uint {
    self.length
  }

  pub fn get<'x>(&'x self, index: uint) -> Option<&'x self::Entry> {
    if index < self.length {
      Some(unsafe { *((self.entries as uint + index * size_of::<&'a self::Entry>()) as *mut &'a self::Entry) })
    } else {
      None
    }
  }
}
