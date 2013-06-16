%define PAGE_PRESENT    (1 << 0)
%define PAGE_WRITE      (1 << 1)	

%define CODE_SEG     0x0008
%define DATA_SEG     0x0010

%define FREE_SPACE 0x9000

use16

org 0x7c00

boot:
	jmp 0x0000:.flush_cs
.flush_cs:
   ; initialize segment registers
   xor ax, ax
   mov ds, ax
   mov es, ax
   mov ss, ax
   ; initialize stack
   mov ax, boot ;;0x7bff
   mov sp, ax
   call check_a20
   cmp ax, 1
   je .a20enabled
   call enable_a20_bios
   call check_a20
   cmp ax, 1
   je .a20enabled
   call enable_a20_keyboard
   call check_a20
   cmp ax, 1
   je .a20enabled
   call enable_a20_fast
   ; if not enabled here, then your computer sucks
	.a20enabled:
   ; load rust code into 0x7e00 so we can jump to it later
   mov ah, 2       ; read
   mov al, 24      ; 24 sectors (12 KiB)
   mov ch, 0       ; cylinder & 0xff
   mov cl, 2       ; sector | ((cylinder >> 2) & 0xc0)
   mov dh, 0       ; head
   mov bx, 0x7e00  ; read buffer
   int 0x13
   jc error
   ;call do_e820
   ; load protected mode GDT and a null IDT (we don't need interrupts)
   ;;cli
	cld
	jmp 0x7e00

; use the INT 0x15, eax= 0xE820 BIOS function to get a memory map
; inputs: es:di -> destination buffer for 24 byte entries
; outputs: bp = entry count, trashes all registers except esi
do_e820:
   xor ebx, ebx		         ; ebx must be 0 to start
   xor bp, bp		            ; keep an entry count in bp
   mov edx, 0x0534D4150	      ; Place "SMAP" into edx
   mov eax, 0xe820
   mov [es:di + 20], dword 1	; force a valid ACPI 3.X entry
   mov ecx, 24		            ; ask for 24 bytes
   int 0x15
   jc short .failed	         ; carry set on first call means "unsupported function"
   mov edx, 0x0534D4150	      ; Some BIOSes apparently trash this register?
   cmp eax, edx		         ; on success, eax must have been reset to "SMAP"
   jne short .failed
   test ebx, ebx		         ; ebx = 0 implies list is only 1 entry long (worthless)
   je short .failed
   jmp short .jmpin
.e820lp:
	mov eax, 0xe820		      ; eax, ecx get trashed on every int 0x15 call
   mov [es:di + 20], dword 1	; force a valid ACPI 3.X entry
   mov ecx, 24		            ; ask for 24 bytes again
   int 0x15
   jc short .e820f		      ; carry set means "end of list already reached"
   mov edx, 0x0534D4150	      ; repair potentially trashed register
.jmpin:
	jcxz .skipent		         ; skip any 0 length entries
	cmp cl, 20		            ; got a 24 byte ACPI 3.X response?
	jbe short .notext
	test byte [es:di + 20], 1	; if so: is the "ignore this data" bit clear?
	je short .skipent
.notext:
	mov ecx, [es:di + 8]	      ; get lower dword of memory region length
	or ecx, [es:di + 12]	      ; "or" it with upper dword to test for zero
	jz .skipent		            ; if length qword is 0, skip entry
	inc bp			            ; got a good entry: ++count, move to next storage spot
	add di, 24
.skipent:
	test ebx, ebx		         ; if ebx resets to 0, list is complete
	jne short .e820lp
.e820f:
	;; mov [mmap_ent], bp	      ; store the entry count
   push bp                    ; store the entry count
	clc			               ; there is "jc" on end of list to this point, so the carry must be cleared
	ret
.failed:
	stc			               ; "function unsupported" error exit
	ret

; Function: check_a20
;
; Purpose: to check the status of the a20 line in a completely self-contained state-preserving way.
;          The function can be modified as necessary by removing push's at the beginning and their
;          respective pop's at the end if complete self-containment is not required.
;
; Returns: 0 in ax if the a20 line is disabled (memory wraps around)
;          1 in ax if the a20 line is enabled (memory does not wrap around)

check_a20:
   pushf
   push ds
   push es
   push di
   push si

   cli

   xor ax, ax ; ax = 0
   mov es, ax

   not ax ; ax = 0xFFFF
   mov ds, ax

   mov di, 0x0500
   mov si, 0x0510

   mov al, byte [es:di]
   push ax

   mov al, byte [ds:si]
   push ax

   mov byte [es:di], 0x00
   mov byte [ds:si], 0xFF

   cmp byte [es:di], 0xFF

   pop ax
   mov byte [ds:si], al

   pop ax
   mov byte [es:di], al

   mov ax, 0
   je .check_a20__exit

   mov ax, 1
   
   .check_a20__exit:
      pop si
      pop di
      pop es
      pop ds
      popf

      ret

enable_a20_bios:
   mov ax, 0x2401
   int 0x15
   ret

use32

enable_a20_keyboard:
   cli

   call a20wait
   mov al, 0xAD
   out 0x64, al

   call a20wait
   mov al, 0xD0
   out 0x64, al

   call a20wait2
   in al, 0x60
   push eax

   call a20wait
   mov al, 0xD1
   out 0x64, al

   call a20wait
   pop eax
   or al, 2
   out 0x60, al

   call a20wait
   mov al, 0xAE
   out 0x64, al

   call a20wait
   sti
   ret

a20wait:
   in al, 0x64
   test al, 2
   jnz a20wait
   ret

a20wait2:
   in al, 0x64
   test al, 1
   jz a20wait2
   ret

use16

enable_a20_fast:
   in al, 0x92
   test al, 2
   jnz .done
   or al, 2
   and al, 0xFE
   out 0x92, al
   .done:
   ret

error:
   mov si, .msg
.loop:
   lodsb
   or al, al
   jz .done
   mov ah, 0x0e
   int 0x10
   jmp .loop
.done:
   jmp $
   .msg db "could not read disk", 0

times 510-($-$$) db 0
db 0x55
db 0xaa

stage2:
	mov edi, FREE_SPACE

; Function to switch directly to long mode from real mode.
; Identity maps the first 2MiB.
; Uses Intel syntax.
 
; es:edi    Should point to a valid page-aligned 16KiB buffer, for the PML4, PDPT, PD and a PT.
; ss:esp    Should point to memory that can be used as a small (1 dword ) stack
 
SwitchToLongMode:
    ; Zero out the 16KiB buffer.
    ; Since we are doing a rep stosd, count should be bytes/4.   
    push di                           ; REP STOSD alters DI.
    mov ecx, 0x1000
    xor eax, eax
    cld
    rep stosd
    pop di                            ; Get DI back.
 
 
    ; Build the Page Map Level 4.
    ; es:di points to the Page Map Level 4 table.
    lea eax, [es:di + 0x1000]         ; Put the address of the Page Directory Pointer Table in to EAX.
    or eax, PAGE_PRESENT | PAGE_WRITE ; Or EAX with the flags - present flag, writable flag.
    mov [es:di], eax                  ; Store the value of EAX as the first PML4E.
 
 
    ; Build the Page Directory Pointer Table.
    lea eax, [es:di + 0x2000]         ; Put the address of the Page Directory in to EAX.
    or eax, PAGE_PRESENT | PAGE_WRITE ; Or EAX with the flags - present flag, writable flag.
    mov [es:di + 0x1000], eax         ; Store the value of EAX as the first PDPTE.
 
 
    ; Build the Page Directory.
    lea eax, [es:di + 0x3000]         ; Put the address of the Page Table in to EAX.
    or eax, PAGE_PRESENT | PAGE_WRITE ; Or EAX with the flags - present flag, writeable flag.
    mov [es:di + 0x2000], eax         ; Store to value of EAX as the first PDE.
 
 
    push di                           ; Save DI for the time being.
    lea di, [di + 0x3000]             ; Point DI to the page table.
    mov eax, PAGE_PRESENT | PAGE_WRITE    ; Move the flags into EAX - and point it to 0x0000.
 
    ; Build the Page Table.
.LoopPageTable:
    mov [es:di], eax
    add eax, 0x1000
    add di, 8
    cmp eax, 0x200000                 ; If we did all 2MiB, end.
    jb .LoopPageTable
 
    pop di                            ; Restore DI.
 
    ; Disable IRQs
    mov al, 0xFF                      ; Out 0xFF to 0xA1 and 0x21 to disable all IRQs.
    out 0xA1, al
    out 0x21, al
    
	nop
    nop
 
    lidt [idtr]                        ; Load a zero length IDT so that any NMI causes a triple fault.

    ; Enter long mode.
	mov eax, cr4
	or eax, 1 << 5
    ; Set the PAE and PGE bit.
    mov cr4, eax
 
    mov edx, edi                      ; Point CR3 at the PML4.
    mov cr3, edx
 
    mov ecx, 0xC0000080               ; Read from the EFER MSR. 
    rdmsr    
 
    or eax, 00000000000000000000000100000001b ;1 << 8                ; Set the LME bit.
    wrmsr
 
    mov ebx, cr0                      ; Activate long mode -
    or ebx, 10000000000000000000000000000001b ;(1 << 31 | 1 << 0)                 ; - by enabling paging and protection simultaneously.
    mov cr0, ebx                    
 
    lgdt [GDT64.Pointer]                ; Load GDT.Pointer defined below.
 
    jmp GDT64.Code:long_mode             ; Load CS with 64 bit segment and flush the instruction cache

long_mode:
   use64
   cli
	jmp $
   ; load all the other segments with 32 bit data segments
   mov ax, GDT64.Data
   mov ds, ax
   mov es, ax
   mov fs, ax
   mov gs, ax
   mov ss, ax
   ; set up stack
   mov rax, 0x7bff
   mov rsp, rax
   ; clear the screen
   ;;mov edi, 0xb8000
   ;;mov ecx, 80*25*2
   ;;mov al, 0
   ;;rep stosb
   ; jump into rust
   jmp end		; temporary

idtr:
   .Length dw 0
   .Base dd 0

trickgdt:
	dw gdt_end - GDT64 - 1 ; size of the GDT
	dd GDT64 ; linear address of GDT

GDT64:                           ; Global Descriptor Table (64-bit).
    .Null: equ $ - GDT64         ; The null descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 0                         ; Access.
    db 0                         ; Granularity.
    db 0                         ; Base (high).
    .Code: equ $ - GDT64         ; The code descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10011000b                 ; Access.
    db 00100000b                 ; Granularity.
    db 0                         ; Base (high).
    .Data: equ $ - GDT64         ; The data descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10010000b                 ; Access.
    db 00000000b                 ; Granularity.
    db 0                         ; Base (high).
    .Pointer:                    ; The GDT-pointer.
    dw $ - GDT64 - 1             ; Limit.
    dq GDT64                     ; Base.

end:

