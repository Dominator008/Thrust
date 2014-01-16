/*
 * Copyright (c) 2014 Arcterus@mail.com
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

pub enum Option<T> {
	Some(T),
	None
}

impl<T> Option<T> {
	#[inline(always)]
	pub fn is_some(&self) -> bool {
		match *self {
			Some(_) => true,
			None => false
		}
	}
	
	#[inline(always)]
	pub fn is_none(&self) -> bool {
		!self.is_some()
	}
}
