mod cpu;
mod peripherals;

use cpu::Cpu;
use peripherals::{Keypad, Framebuffer};

pub struct Chip8 {
    pub cpu: Cpu,

    pub display: Framebuffer,
    pub keypad: Keypad,
}


impl Chip8 {



}



