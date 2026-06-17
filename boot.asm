bits 32
global start
extern _start

section .multiboot
    align 4
    dd 0x1BADB002            ; Magic Number
    dd 0x00                  ; Flags auf 0, um GRUB die Arbeit zu erleichtern
    dd - (0x1BADB002 + 0x00) ; Checksumme

section .text
start:
    cli                      ; Interrupts aus
    mov esp, stack_space     ; Stack setzen
    
    call _start              ; Ab in den Rust-Kernel

    cli
.hang:
    hlt
    jmp .hang

section .bss
    align 16
    resb 8192                ; 8 KB Stack
stack_space:
