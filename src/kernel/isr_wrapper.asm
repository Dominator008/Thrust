use64
global int_handler_kbd_wrapper
align 8

extern _interrupt_handler_kbd

int_handler_kbd_wrapper:
  push rax
  push rcx
  push rdx
  push rsi
  push rdi
  push r8
  push r9
  push r10
  push r11
  mov rdi, rsp
  add rdi, 72
  call  _interrupt_handler_kbd
  pop r11
  pop r10
  pop r9
  pop r8
  pop rdi
  pop rsi
  pop rdx
  pop rcx
  pop rax
  iretq
