#[author = "Arcterus"];
#[license = "MPL v2.0"];

pub trait One {
	fn one() -> Self;
}

pub trait Times {
	fn times(&self, it: &fn());
}
