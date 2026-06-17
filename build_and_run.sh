#!/bin/bash
set -e 

echo "[1] Aufräumen..."
rm -rf isofiles neural_os.iso *.o *.bin

echo "[2] Assemblieren..."
nasm -f elf32 boot.asm -o boot.o

echo "[3] Rust kompilieren (Bare-Metal Modus)..."
# HIER IST DER FIX: Wir verbieten PIC und nutzen ein statisches Speichermodell
rustc --target i686-unknown-linux-gnu -C panic=abort -C relocation-model=static -O --emit obj \
      --crate-type staticlib src/main.rs -o main.o

echo "[4] Linken..."
ld -m elf_i386 -T linker.ld -o neural_os.bin boot.o main.o

echo "[5] ISO generieren..."
mkdir -p isofiles/boot/grub
cp neural_os.bin isofiles/boot/
echo 'menuentry "Neural OS" { multiboot /boot/neural_os.bin; boot }' > isofiles/boot/grub/grub.cfg
grub-mkrescue -o neural_os.iso isofiles 2>/dev/null

echo "[6] QEMU starten..."
qemu-system-x86_64 -cdrom neural_os.iso -vga std
