#[author = "Arcterus"];
#[license = "MPL v2.0"];

pub static SCREEN: *mut super::Screen = 0xb8000 as *mut super::Screen;
pub static MAX_ROW: uint = 25;
pub static MAX_COLUMN: uint = 80;
pub static SCREEN_SIZE: uint = MAX_COLUMN * MAX_ROW;

