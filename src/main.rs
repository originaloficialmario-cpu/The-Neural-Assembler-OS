#![no_main] // Sag Rust: "Such nicht nach einer main-Funktion!"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Schreibe ein 'A' in den Grafikspeicher (Bildschirm)
    unsafe {
        let vga_buffer = 0xB8000 as *mut u8;
        *vga_buffer = b'A'; 
        *vga_buffer.add(1) = 0x07; // Weiße Schrift auf schwarzem Grund
    }
    loop {}
}
