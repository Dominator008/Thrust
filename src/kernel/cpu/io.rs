#[inline(always)]
pub unsafe fn out<T>(port: u16, val: T) {
  asm!("out $1, $0" :: "{al}"(val), "{dx}"(port) :: "intel");
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
  let ret: u8;
  asm!("inb %dx, %al"
      : "={al}"(ret)
      : "{dx}"(port)
      :
      : "volatile" );
  ret
}

pub unsafe fn wait(port: u16, mask: u8) {
  while inb(port) & mask != 0 {}
}
