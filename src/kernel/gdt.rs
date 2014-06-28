use stdio;

/* Defines an GDT entry */
#[packed]
pub struct GDTEntry {
  limit_lo: u16,
  base_lo: u16,
  base_mid: u8,
  access: u8,
  granularity: u8,
  base_hi: u8
}

/** Defines an GDT pointer */
#[packed]
pub struct GDTPointer {
  limit: u16,
  base: u64
}

/** Declare an GDT of 5 entries. */
#[no_mangle]
pub static mut gdt: [u64, ..5] = [0, ..5];
/*pub static mut gdt: [GDTEntry, ..5] = [GDTEntry {
    limit_lo: 0, base_lo: 0, base_mid: 0, access: 0, granularity: 0, base_hi: 0
}, ..5];*/

#[no_mangle]
pub static mut gdtp: GDTPointer = GDTPointer {limit: 0, base: 0};

#[no_mangle]
/*fn gdt_set_gate(num: uint, base: u32, limit: u32, access: u8, granularity: u8) {
  unsafe {
    gdt[num as uint].base_lo = (base & 0xFFFF) as u16;
    gdt[num as uint].base_mid = ((base >> 16) & 0xFF) as u8;
    gdt[num as uint].base_hi = ((base >> 24) & 0xFF) as u8;

    gdt[num as uint].limit_lo = (limit & 0xFFFF) as u16;
    gdt[num as uint].granularity = ((limit >> 16) & 0x0F) as u8;

    gdt[num as uint].granularity |= (granularity & 0xF0) as u8;
    gdt[num as uint].access = access;
  }
}*/

/*extern {
  fn load_gdt(gdtp: GDTPointer, cd_offset: u32, dd_offset: u32);
}*/

#[no_mangle]
pub unsafe fn install_gdt() {
  gdtp.limit = ((super::core::mem::size_of::<u64>() * 5) - 1) as u16;
  gdtp.base = &gdt as *[u64, ..5] as u64;

  /*gdt_set_gate(0, 0, 0, 0, 0);                // Null segment
  gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF); // Kernel code segment
  gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF); // Kernel data segment
  gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF); // User mode code segment
  gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF); // User mode data segment*/

let GDT_CS =   (0x00180000000000); /*** code segment descriptor ***/
let GDT_DS =     (0x00100000000000); /*** data segment descriptor ***/

let DPL0  =        (0x00000000000000); /*** descriptor privilege level 0 ***/
let P       =    (0x00800000000000); /*** present ***/
let L      =       (0x20000000000000);  /*** long mode ***/
let W      =       (0x00020000000000); /***writable data segment ***/
    gdt[0] = 0;             // NULL descriptor
    gdt[1] = GDT_CS | P | DPL0 | L;  /*** kernel code segment descriptor ***/
    gdt[2] = GDT_DS | P | W;   /*** kernel data segment descriptor ***/

  stdio::print_long(&gdtp as *GDTPointer as u64);
  /* Load GDT */
  //load_gdt(gdtp, 8, 16);
  asm!("  lgdtq ($0) \n
          pushq %rsi \n
          movabsq $$1f, %r10 \n
          pushq %r10 \n
          lretq \n
        1: \n
          movq $1, %es \n
          movq $1, %fs \n
          movq $1, %gs \n
          movq $1, %ds \n
          movq $1, %ss \n
          retq"
      :: "r" (gdtp), "r" (8)
      );
}
