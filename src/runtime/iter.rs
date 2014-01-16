/*
 * Copyright (c) 2014 Arcterus@mail.com
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 
use super::zero::*;
use super::num::*;

#[inline]
pub fn iterate<A: Add<A, A> + Ord + One>(start: A, end: A, it: |&A| -> bool) -> bool {
	iterate_step(start, end, One::one(), it)
}

pub fn iterate_step<A: Add<A, A> + Ord + One>(start: A, end: A, step: A, it: |&A| -> bool) -> bool {
	let mut start = start;
	while start < end {
		if !it(&start) {
			return false;
		}
		start = start + step;
	}
	true
}

#[inline]
pub fn range<A: Add<A, A> + Ord + One>(start: A, end: A, it: |&A| -> () ) {
	range_step(start, end, One::one(), it);
}

#[inline]
pub fn range_inclusive<A: Add<A, A> + Ord + One>(start: A, end: A, it: |&A| -> () ) {
	range_step_inclusive(start, end, One::one(), it);
}

pub fn range_step<A: Add<A, A> + Ord + One>(start: A, end: A, step: A, it: |&A| -> () ) {
	let mut start = start;
	while start < end {
		it(&start);
		start = start + step;
	}
}

#[inline]
pub fn range_step_inclusive<A: Add<A, A> + Ord + One>(start: A, end: A, step: A, it: |&A| -> () ) {
	range_step(start, end + One::one(), step, it);
}
