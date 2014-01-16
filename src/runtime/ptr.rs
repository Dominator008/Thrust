/*
 * Copyright (c) 2014 Arcterus@mail.com
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

use runtime::zero::{Eq};

impl<T> Eq for *T {
	#[inline(always)]
	fn eq(&self, other: &*T) -> bool {
		(*self as uint) == (*other as uint)
	}

	#[inline(always)]
	fn ne(&self, other: &*T) -> bool {
		!self.eq(other)
	}
}

/*impl<T> *T {
	pub fn is_null<T>(&self) -> bool {
		to_const_unsafe_ptr(self) == null()
	}
}*/

#[inline(always)]
pub fn null<T>() -> *T {
	0 as *T
}

#[inline(always)]
pub fn mut_null<T>() -> *mut T {
	0 as *mut T
}

#[inline(always)]
pub fn is_null<T>(ptr: *T) -> bool {
	ptr == null()
}

#[inline(always)]
pub fn is_not_null<T>(ptr: *T) -> bool {
	!is_null(ptr)
}

#[inline(always)]
pub fn to_const_unsafe_ptr<T>(thing: &T) -> *T {
	thing as *T
}

#[inline(always)]
pub fn to_mut_unsafe_ptr<T>(thing: &mut T) -> *mut T {
	thing as *mut T
}

#[inline(always)]
pub fn to_unsafe_ptr<T>(thing: &T) -> *T {
	thing as *T
}
