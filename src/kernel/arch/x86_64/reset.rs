/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

#[inline(always)]
pub unsafe fn immediate_reset() -> ! {
	asm!("xor %eax, %eax; lidt $0; int3" :: "m" (0 as *u64));
	loop {}
}

#[inline(always)]
#[no_mangle]
pub unsafe fn __morestack() -> ! {
	//reset::panic("cannot use __morestack from the kernel"); // TODO: get this to work
	immediate_reset();
}
