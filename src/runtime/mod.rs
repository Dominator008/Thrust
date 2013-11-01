#[author = "Arcterus"];
#[license = "MPL v2.0"];

use self::zero::size_of;
use super::console;

pub mod zero;
pub mod ptr;
pub mod option;
pub mod result;
pub mod vec;
pub mod uint;
pub mod iter;
pub mod device;
pub mod num;
pub mod str;

// Failure

#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
	unsafe {
		console::color_print("Error", console::Red, console::BACKGROUND_COLOR);
		console::print(": failed (");
		console::print_bytes(expr);
		console::print(") at line ");
		//console::print(line.to_str());
		console::print(" in file ");
		console::print_bytes(file);
		zero::abort()
	}
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(_: *i8, _: uint, _: uint, _: uint) {
    unsafe {
        zero::abort()
    }
}

// Memory

#[no_mangle]
pub unsafe fn memcmp(ptr1: *u8, ptr2: *u8, size: uint) -> i32 {
	let mut mptr1 = ptr1;
	let mut mptr2 = ptr2;
	let mut msize = size;
	while msize > 0 {
		let val = (*mptr1 as i32) - (*mptr2 as i32);
		if val != 0 {
			return val;
		}
		mptr1 = ((mptr1 as uint) + size_of::<*u8>()) as *u8;
		mptr2 = ((mptr2 as uint) + size_of::<*u8>()) as *u8;
		msize -= 1;
	}
	0i32
}

#[no_mangle]
pub unsafe fn memcpy(dest: *mut u8, src: *u8, num: uint) -> *u8 {
	let mut msrc = src;
	let mut mnum = num;
	let mut mdest = dest;
	while mnum > 0 {
		*mdest = *msrc;
		mdest = ((mdest as uint) + size_of::<*mut u8>()) as *mut u8;
		msrc = ((msrc as uint) + size_of::<*u8>()) as *u8;
		mnum -= 1;
	}
	mdest as *u8
}
