pub trait One {
	fn one() -> Self;
}

pub trait Times {
	fn times(&self, it: || -> () );
}
