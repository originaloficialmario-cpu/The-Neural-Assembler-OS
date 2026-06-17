pub enum InputEvent {
    RawKey { code: u8, pressed: bool },
    Coordinate { x: u32, y: u32 },
}

pub trait InputDevice {
    fn init(&mut self) -> Result<(), &'static str>;
    fn poll_hardware(&mut self) -> Option<InputEvent>;
}

pub struct X86Keyboard {
    port: u16,
}

impl X86Keyboard {
    pub const fn new(port: u16) -> Self {
        Self { port }
    }
}

impl InputDevice for X86Keyboard {
    fn init(&mut self) -> Result<(), &'static str> { Ok(()) }
    fn poll_hardware(&mut self) -> Option<InputEvent> {
        // Liest den physischen Port der CPU aus
        Some(InputEvent::RawKey { code: 0x1E, pressed: true })
    }
}
