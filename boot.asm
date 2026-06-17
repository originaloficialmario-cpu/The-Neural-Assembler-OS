bits 32
global start
global keyboard_handler_asm
extern _start
extern keyboard_handler

section .multiboot
    align 4
    dd 0x1BADB002            ; Magic Number
    dd 0x00                  ; Flags
    dd - (0x1BADB002 + 0x00) ; Checksumme

section .text
start:
    cli                      ; Interrupts blockieren während des Setups
    mov esp, stack_space     ; Kernel-Stack aufsetzen
    
    call _start              ; Ab in den Rust-Kernel

    cli
.hang:
    hlt
    jmp .hang

; Der Hardware-Interrupt-Tunnel für die Tastatur
keyboard_handler_asm:
    pushad                   ; Alle Allzweckregister auf dem Stack sichern
    call keyboard_handler    ; Den Rust-Code ausführen
    popad                    ; Alle Register wiederherstellen
    iretd                    ; Interrupt-Rücksprung (wichtig: iretd, nicht ret!)

section .bss
    align 16
    resb 8192                ; 8 KB Stack
stack_space:
