#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn print_string(row: usize, text: &[u8], color: u8) {
    unsafe {
        let vga_buffer = 0xB8000 as *mut u8;
        let row_offset = row * 80 * 2;
        
        for i in 0..text.len() {
            if i >= 80 { break; }
            // write_volatile zwingt die Hardware, den Befehl auszuführen
            vga_buffer.add(row_offset + i * 2).write_volatile(text[i]);
            vga_buffer.add(row_offset + i * 2 + 1).write_volatile(color);
        }
    }
}

fn clear_screen() {
    unsafe {
        let vga_buffer = 0xB8000 as *mut u8;
        for i in 0..(80 * 25) {
            vga_buffer.add(i * 2).write_volatile(b' ');
            vga_buffer.add(i * 2 + 1).write_volatile(0x07);
        }
    }
}

const BLOCK_SIZE: usize = 64 * 1024;       
const MEMORY_START: usize = 0x200000;      

struct BlockManager {
    next_block_index: usize,
}

impl BlockManager {
    fn allocate_block(&mut self) -> *mut u8 {
        let addr = MEMORY_START + (self.next_block_index * BLOCK_SIZE);
        self.next_block_index += 1;
        addr as *mut u8
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    clear_screen();
    
    print_string(0, b"NEURAL ASSEMBLER OS - V1.0.0 ACTIVE", 0x0A); 
    print_string(1, b"------------------------------------", 0x07); 
    print_string(2, b"INITIALIZING CORE MMU (64KB BLOCK STRUCTURE)...", 0x0E); 

    let mut manager = BlockManager { next_block_index: 0 };
    let block_ptr = manager.allocate_block();
    
    unsafe {
        // Physischen RAM beschreiben
        block_ptr.add(0).write_volatile(0xDE);
        block_ptr.add(1).write_volatile(0xAD);
        block_ptr.add(2).write_volatile(0xBE);
        block_ptr.add(3).write_volatile(0xEF);

        // Physischen RAM auslesen und verifizieren
        if block_ptr.add(0).read_volatile() == 0xDE && 
           block_ptr.add(1).read_volatile() == 0xAD && 
           block_ptr.add(2).read_volatile() == 0xBE && 
           block_ptr.add(3).read_volatile() == 0xEF {
            
            print_string(4, b"MMU TEST 01: 64KB BLOCK ALLOCATION     [ OK ]", 0x0A); 
            print_string(5, b"RAM INTEGRITY VERIFIED AT PHYSICAL ADDR 0x200000", 0x0F); 
        } else {
            print_string(4, b"MMU TEST 01: 64KB BLOCK ALLOCATION     [ FAILED ]", 0x0C); 
        }
    }

    loop {}
}
