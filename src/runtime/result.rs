/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

pub enum Result<T, U> {
	Ok(T),
	Err(U)
}

impl<T, U> Result<T, U> {
	#[inline(always)]
	pub fn is_ok(&self) -> bool {
		match *self {
			Ok(_) => true,
			Err(_) => false
		}
	}
	
	#[inline(always)]
	pub fn is_err(&self) -> bool {
		!self.is_ok()
	}
}
