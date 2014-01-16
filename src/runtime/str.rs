use super::iter;
use super::zero::transmute;

pub trait Str {
	fn as_slice<'a>(&'a self) -> &'a str;
}

pub trait StrSlice {
	fn len(&self) -> uint;
	fn each_byte(&self, it: |u8| -> bool) -> bool;
}

impl<'a> Str for &'a str {
	fn as_slice<'a>(&'a self) -> &'a str {
		*self
	}
}

impl Str for ~str {
	fn as_slice<'a>(&'a self) -> &'a str {
		let s: &'a str = *self;
		s
	}
}

impl<'a> StrSlice for &'a str {
	#[inline]
	fn len(&self) -> uint {
		as_buf(*self,|_, size| {
			size
		})
	}

	fn each_byte(&self, it: |u8| -> bool) -> bool {
		iter::iterate(0, self.len(),|i| { it(self[*i]) })
	}
}

pub fn as_buf<T>(s: &str, func: |*u8, uint| -> T) -> T {
	unsafe {
		let v: *(*u8, uint) = transmute(&s);
		let (buf, len) = *v;
		func(buf, len)
	}
}

