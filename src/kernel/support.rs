/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod raw {
	extern {
		#[link_name = "llvm.debugtrap"]
		pub fn breakpoint();
	}
}

#[no_mangle]
pub fn breakpoint() {
	unsafe { raw::breakpoint(); }
}

