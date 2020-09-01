use super::peripherals::{Framebuffer, Keypad};
use rand::prelude::*;


#[derive(Clone)]
pub struct Cpu {
    ram: [u8; 4069],
    v: [u8; 16],
    i: u16,
    pc: u16,

    dt: u8,
    st: u8,
    
    rng : rand::rngs::ThreadRng,
    stack: Vec<u16>,

}



impl Cpu {
    pub fn new() -> Self {
        let mut ram = [0; 4069];

        ram[..80].copy_from_slice( &[
            0xF0, 0x90, 0x90, 0x90, 0xF0,
            0x20, 0x60, 0x20, 0x20, 0x70,
            0xF0, 0x10, 0xF0, 0x80, 0xF0,
            0xF0, 0x10, 0xF0, 0x10, 0xF0,
            0x90, 0x90, 0xF0, 0x10, 0x10,
            0xF0, 0x80, 0xF0, 0x10, 0xF0,
            0xF0, 0x80, 0xF0, 0x90, 0xF0,
            0xF0, 0x10, 0x20, 0x40, 0x40,
            0xF0, 0x90, 0xF0, 0x90, 0xF0,
            0xF0, 0x90, 0xF0, 0x10, 0xF0,
            0xF0, 0x90, 0xF0, 0x90, 0x90,
            0xE0, 0x90, 0xE0, 0x90, 0xE0,
            0xF0, 0x80, 0x80, 0x80, 0xF0,
            0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0,
            0xF0, 0x80, 0xF0, 0x80, 0x80,
        ]);

        let v = [0; 16];
        let i = 0;
        let pc = 0x200;
        
        let dt = 0;
        let st = 0;

        Cpu {
            ram,
            v,
            i,
            pc,
            dt,
            st,
            rng: rand::thread_rng(),
            stack: Vec::new(),
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt-= 1;
        }
        if self.st > 0 {
            self.st-=1;
        }
    }

    pub fn execute_cycle(&mut self, framebuffer: &mut Framebuffer, keypad: &mut Keypad) {
        let byte1 = self.ram[self.pc as usize];
        let byte2 = self.ram[(self.pc+1) as usize];
        

        let opcode = ( (byte1 & 0xF0) >> 4, byte1 & 0x0F, (byte2 & 0xF0) >> 4, byte2 & 0x0F );
        // println!("{:X?}", opcode);
        let x = byte1 as usize & 0x0F;
        let y = (byte2 as usize & 0xF0) >> 4;
        let kk = byte2;
        let n = byte2 & 0x0F;
        let nnn = ((byte1 as u16 & 0xF) << 8) | byte2 as u16;
        // println!("{:X?}", byte1); 
        self.pc+=2;
        

        match opcode {
            (0,0,0xE,  0) => framebuffer.clear(),
            (0,0,0xE,0xE) => self.pc = self.stack.pop().unwrap(),
            (0, _, _, _ ) => {  },
            (1, _, _, _ ) => self.pc = nnn,
            (2, _, _, _ ) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            },
            (3, _, _, _ ) => if self.v[x] == kk{
                self.pc+=2;
            },
            (4, _, _, _ ) => if self.v[x] != kk {
                self.pc+=2;
            },
            (5, _, _, 0 ) => if self.v[x] == self.v[y] {
                self.pc+=2;
            },
            (6, _, _, _ ) => self.v[x] = kk,
            (7, _, _, _ ) => self.v[x] = self.v[x].wrapping_add(kk),
            (8, _, _, 0 ) => self.v[x] = self.v[y],
            (8, _, _, 1 ) => self.v[x] |= self.v[y],
            (8, _, _, 2 ) => self.v[x] &= self.v[y],
            (8, _, _, 3 ) => self.v[x] ^= self.v[y],
            (8, _, _, 4 ) => {
                let (result, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = carry as u8;
                self.v[x] = result;
            },
            (8, _, _, 5 ) => {
                let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = !borrow as u8;
                self.v[x] = result;
            },
            (8, _, _, 6 ) => {
                self.v[0xF] = self.v[x] % 2;
                self.v[x] /= 2;
            }
            (8, _, _, 7 ) => {
                let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = !borrow as u8;
                self.v[x] = result;
            },
            (8, _, _,0xE ) => {
                self.v[0xF] = self.v[x]>>7;
                self.v[x] = self.v[x]<<1;
            },
            (9, _, _, 0 ) => if self.v[x] != self.v[y] {
                self.pc+=2;
            },
            (0xA, _, _, _) => self.i = nnn,
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
            (0xC, _, _, _) => self.v[x] = self.rng.gen::<u8>() & kk,
            (0xD, _, _, _) => self.v[0xF] = framebuffer.draw(
                    (self.v[x], self.v[y]),
                    &self.ram[self.i as usize..(self.i+n as u16) as usize]
                ) as u8,
            (0xE, _, 9,0xE) => if keypad.keys[self.v[x] as usize] {
                self.pc+=2; 
            },
            (0xE, _, 0xA,1) => if !keypad.keys[self.v[x] as usize] {
                self.pc+=2;
            },
            (0xF, _, 0,7) => self.v[x] = self.dt,
            (0xF, _, 0,0xA) => match keypad.first_pressed_key() {      
                Some(key) => self.v[x] = key,
                None => self.pc-=2,
            },
            (0xF, _, 1, 5) => self.dt = self.v[x],
            (0xF, _, 1, 8) => self.st = self.v[x],
            (0xF, _, 1,0xE) => self.i+= self.v[x] as u16,
            (0xF, _, 2, 9) => self.i = 5*self.v[x] as u16,
            (0xF, _, 3, 3) => {
                self.ram[self.i as usize+0] = self.v[x]/100;
                self.ram[self.i as usize+1] = (self.v[x]/10)%10;
                self.ram[self.i as usize+2] = self.v[x]%10;
            },
            (0xF, _, 5, 5) => for idx in 0usize..(x+1) {
                self.ram[self.i as usize+idx] = self.v[idx];
            },
            (0xF, _, 6, 5) => for idx in 0usize..(x+1) {
                self.v[idx]=self.ram[self.i as usize +idx];
            },
            _ => panic!("invalid opcode"),
        }

    }
    
    pub fn load_rom(&mut self, rom: &[u8]) {
        
        let start = 0x200;
        let end = 0x200+rom.len();

        self.pc = 0x200;
        self.i = 0;
        self.v = [0;16];
        self.dt =0;
        self.st = 0;
        self.stack.clear();
        self.ram[start..end].copy_from_slice(rom);
    }



}
