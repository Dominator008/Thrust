/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

pub mod reset;

#[inline(always)]
#[no_mangle]
pub unsafe fn __morestack() -> ! {
	   super::error::panic("cannot use __morestack from the kernel");
}

