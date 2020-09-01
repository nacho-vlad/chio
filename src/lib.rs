mod cpu;
mod peripherals;

use cpu::Cpu;
use peripherals::{Keypad, Framebuffer};
use std::{
    path::Path,
    fs::File,
    io::Read,
};

#[derive(Clone)]
pub struct Chip8 {
    cpu: Cpu,

    pub framebuffer: Framebuffer,
    pub keypad: Keypad,
}


impl Chip8 {
    
    pub fn new() -> Self {
        let cpu = Cpu::new();
        let framebuffer = Framebuffer::new();
        let keypad = Keypad::new();
        
        Self {
            cpu,
            framebuffer,
            keypad,
        }
    }


    pub fn load_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        
        self.cpu.load_rom(&buf);
        Ok(())
    }


    pub fn execute_cycle(&mut self) {
        self.cpu.execute_cycle(&mut self.framebuffer, &mut self.keypad);
    }

    pub fn decrement_timers(&mut self) {
        self.cpu.decrement_timers();
    }

    pub fn framebuffer(&self) -> Framebuffer {
        self.framebuffer
    }
}



