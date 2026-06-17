pub struct VramSectorHandle {
    pub ptr: *mut u8,
    pub limit: usize,
}

pub trait VolatileMemoryManager {
    fn alloc_64kb_sector(&mut self) -> Result<VramSectorHandle, &'static str>;
    fn absolute_flush(&mut self, handle: &VramSectorHandle);
}

pub struct MockHardwareMemory {
    arena: [u8; 65536], // Das unnachgiebige 64KB-Mindset
}

impl MockHardwareMemory {
    pub const fn new() -> Self {
        Self { arena: [0; 65536] }
    }
}

impl VolatileMemoryManager for MockHardwareMemory {
    fn alloc_64kb_sector(&mut self) -> Result<VramSectorHandle, &'static str> {
        Ok(VramSectorHandle {
            ptr: self.arena.as_mut_ptr(),
            limit: 65536,
        })
    }

    fn absolute_flush(&mut self, handle: &VramSectorHandle) {
        unsafe {
            core::ptr::write_bytes(handle.ptr, 0, handle.limit);
        }
    }
}
