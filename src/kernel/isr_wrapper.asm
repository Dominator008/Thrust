use32
global  int_handler_kbd_wrapper
align   4

extern _interrupt_handler_kbd

int_handler_kbd_wrapper:
  pushad
  mov ax, 0x08
  mov gs, ax
  call  _interrupt_handler_kbd
  popad
  iret
