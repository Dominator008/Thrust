#[author = "Arcterus"];
#[license = "MPL v2.0"];

use super::iter;
use super::zero::transmute;

pub trait Str {
	fn as_slice<'a>(&'a self) -> &'a str;
}

pub trait StrSlice {
	fn len(&self) -> uint;
	fn each_byte(&self, it: &fn(u8) -> bool) -> bool;
}

impl<'self> Str for &'self str {
	fn as_slice<'a>(&'a self) -> &'a str {
		*self
	}
}

impl<'self> Str for ~str {
	fn as_slice<'a>(&'a self) -> &'a str {
		let s: &'a str = *self;
		s
	}
}

impl<'self> StrSlice for &'self str {
	#[inline]
	fn len(&self) -> uint {
		do as_buf(*self) |_, size| {
			size
		}
	}

	fn each_byte(&self, it: &fn(u8) -> bool) -> bool {
		do iter::iterate(0, self.len()) |i| { it(self[*i]) }
	}
}

pub fn as_buf<T>(s: &str, func: &fn(*u8, uint) -> T) -> T {
	unsafe {
		let v: *(*u8, uint) = transmute(&s);
		let (buf, len) = *v;
		func(buf, len)
	}
}

