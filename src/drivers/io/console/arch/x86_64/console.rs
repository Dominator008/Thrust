pub static SCREEN_ADDR: uint = 0xb8000;
pub static MAX_ROW: uint = 25;
pub static MAX_COLUMN: uint = 80;

pub fn move_cursor(row: uint, col: uint) {
	let pos = row * 80 + col;
	unsafe {
		asm!("
			mov al, 0xF
			mov dx, 0x3D4
			out dx, al

			mov ax, bx
			mov dx, 0x3D5
			out dx, al

			mov al, 0xE
			mov dx, 0x3D4
			out dx, al

			mov ax, bx
			shr ax, 8
			mov dx, 0x3D5
			out dx, al
		" : : "{bx}" (pos) : "al", "dx": "intel");
	}
}
