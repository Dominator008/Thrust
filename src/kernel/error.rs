#[author = "Arcterus"];
#[license = "MPL v2.0"];

use super::console;

pub fn panic(reason: &str) {
	console::color_println("Ohs noes!  You've been harpooned!", console::Red, console::BACKGROUND_COLOR);
	// print fail whale (kraken?)
	console::print("Reason: ");
	console::print(reason);
	// wait 10 seconds
	//unsafe { super::immediate_reset(); }
}
