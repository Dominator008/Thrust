#[author = "Arcterus"];
#[license = "MPL v2.0"];

pub unsafe fn immediate_reset() {
	asm!("xor %eax, %eax; lidt $0; int3" :: "m" (0 as *u64));
}
