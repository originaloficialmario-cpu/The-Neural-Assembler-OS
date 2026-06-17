; boot.asm - Der Zündschlüssel für unser Neural OS

; 1. Der "Hallo QEMU"-Block (Multiboot Header)
section .multiboot
align 4
dd 0x1BADB002             ; Die Magic Number: Sagt der Hardware "Ich bin ein OS!"
dd 0x00                   ; Flags
dd - (0x1BADB002 + 0x00)  ; Checksumme (Beweist, dass die Magic Number fehlerfrei ist)

; 2. Etwas Platz im Speicher (RAM) für den Anfang reservieren
section .bss
align 16
stack_bottom:
resb 4096                 ; 4 Kilobyte Speicher für Notizen der CPU
stack_top:

; 3. Der eigentliche Start-Code
section .text
global start
extern _start             ; Das ist der Link zu deiner Rust-Datei (main.rs)

start:
    ; Dem Prozessor sagen, wo er seine 4 KB Speicher findet
    mov esp, stack_top

    ; JETZT GEHT'S LOS: Übergabe an deinen Rust-Kernel!
    call _start

    ; Falls dein Rust-Code sich jemals beenden sollte, 
    ; lassen wir den Prozessor hier anhalten, damit er nicht abstürzt.
    cli
.hang:
    hlt
    jmp .hang
