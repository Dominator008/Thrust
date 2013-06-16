#[author = "Arcterus"];
#[license = "MPL v2.0"];

pub trait Times {
	fn times(&self, it: &fn() -> bool) -> bool;
}
