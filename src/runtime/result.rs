#[author = "Arcterus"];
#[license = "MPL v2.0"];

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
