#[author = "Arcterus"];
#[license = "MPL v2.0"];

use runtime::iter;

pub fn iterate(low: uint, high: uint, it: &fn(uint) -> bool) -> bool {
	iterate_step(low, high, 1, it)
}

pub fn iterate_step(low: uint, high: uint, step: uint, it: &fn(uint) -> bool) -> bool {
	let mut i = low;
	while i < high {
		if !it(i) {
			return false;
		}
		i += step;
	}
	true
}

impl iter::Times for uint {
	#[inline(always)]
	fn times(&self, it: &fn() -> bool) -> bool {
		iterate(0, *self, |_| { it() })
	}
}
