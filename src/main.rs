#![no_std]
#![no_main]

mod hal;
mod core_brain;

use core::panic::PanicInfo;
use hal::memory::MockHardwareMemory;
use hal::input::X86Keyboard;
use core_brain::NeuralRuntime;

// --- GLOBALE VARIABLEN ---
static mut BUFFER_INDEX: usize = 0;

// --- EXPORTIERTE FUNKTION FÜR DEN LINKER (Der Tunnel) ---
#[no_mangle]
pub extern "C" fn keyboard_handler() {
    unsafe {
        // Hier wird das Signal vom boot.asm abgefangen
        BUFFER_INDEX = 1; 
    }
}

// --- HILFSFUNKTIONEN ---
fn print_char_at(x: usize, y: usize, c: u8, color: u8) {
    unsafe {
        let vga_buffer = 0xB8000 as *mut u8;
        let offset = (y * 80 + x) * 2;
        vga_buffer.add(offset).write_volatile(c);
        vga_buffer.add(offset + 1).write_volatile(color);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut memory_manager = MockHardwareMemory::new();
    let mut keyboard = X86Keyboard::new(0x60);
    let mut runtime = NeuralRuntime::new(&mut memory_manager, &mut keyboard);

    // Bildschirm leeren
    unsafe {
        let vga_buffer = 0xB8000 as *mut u8;
        for i in 0..(80 * 25) { vga_buffer.add(i * 2).write_volatile(b' '); }
    }
    
    // Statusmeldung
    let msg = b"NEURAL ASSEMBLER OS READY";
    for i in 0..msg.len() {
        print_char_at(i, 0, msg[i], 0x0A);
    }

    loop {
        // Hier läuft die Event-Schleife
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
