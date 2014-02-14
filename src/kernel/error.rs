/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

use super::console;

pub fn panic(reason: &str) {
	console::color_println("Ohs noes!  You've been harpooned!", console::Red, console::BACKGROUND_COLOR);
	// print fail whale (kraken?)
	console::print("Reason: ");
	console::print(reason);
	// wait 10 seconds
//	abort();
}

#[no_mangle]
#[inline(always)]
pub fn abort() -> ! {
	unsafe { super::reset::immediate_reset(); }
}
