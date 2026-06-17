; boot.asm - Der Zündschlüssel für unser Neural OS

; 1. Der "Hallo QEMU"-Block (Multiboot Header)
section .multiboot
align 4
dd 0x1BADB002             ; Die Magic Number: Sagt der Hardware "Ich bin ein OS!"
dd 0x00                   ; Flags
dd - (0x1BADB002 + 0x00)  ; Checksumme

; 2. Etwas Platz im Speicher (RAM) reservieren
section .bss
align 16
stack_bottom:
resb 4096                 ; 4 KB Speicher
stack_top:

; 3. Der eigentliche Start-Code
section .text
global start
extern _start             ; Link zu main.rs

start:
    mov esp, stack_top
    call _start           ; Übergabe an den Rust-Kernel!
    cli
.hang:
    hlt
    jmp .hang
