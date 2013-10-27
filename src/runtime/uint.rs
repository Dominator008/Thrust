#[author = "Arcterus"];
#[license = "MPL v2.0"];

use super::iter;
use super::num;
use super::zero;

impl num::One for uint {
	#[inline]
	fn one() -> uint {
		1u
	}
}

impl zero::Add<uint, uint> for uint {
	#[inline]
	fn add(&self, rhs: &uint) -> uint {
		*self + *rhs
	}
}

impl zero::Ord for uint {
	#[inline]
	fn lt(&self, other: &uint) -> bool { *self < *other }
	#[inline]
	fn le(&self, other: &uint) -> bool { *self <= *other }
	#[inline]
	fn gt(&self, other: &uint) -> bool { *self > *other }
	#[inline]
	fn ge(&self, other: &uint) -> bool { *self >= *other }
}

impl num::Times for uint {
	#[inline]
	fn times(&self, it: &fn()) {
		iter::range(0, *self, |_| { it(); });
	}
}

