; load a new GDT
; parameter 1: address of gdtr (rdi)
; parameter 2: new code descriptor offset (rdx)
; parameter 3: new data descriptor offset (rcx)

use64
global load_gdt
align 8

load_gdt:
  lgdt [rdi]
  push rsi       ; push code selector
  mov r10, done
  push r10       ; push return address
  o64 retf       ; far-return to new cs descriptor ( the ret below )
done:
  jmp $
  mov es, rdx
  mov fs, rdx
  mov gs, rdx
  mov ds, rdx
  mov ss, rdx
  ret
