use std::option::Option;

#[derive(Copy,Clone,Debug)]
pub struct Framebuffer {
    framebuffer: [u64; 32]
}

const FIRST_BIT :u64 = 1<<63;

impl Framebuffer {
    pub fn new() -> Self {
        Framebuffer {
            framebuffer: [0; 32],
        }
    }

    pub fn rows(&self) -> [Row; 32] {
        let mut rows = [0.into(); 32];
        for (i, &row) in self.framebuffer.iter().enumerate() {
            rows[i] = row.into();
        }
        rows
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        self.framebuffer[y as usize] & (FIRST_BIT>>x) != 0 
    }

    pub fn draw(&mut self, coords: (u8,u8) , sprite: &[u8]) -> bool {
        let mut collision = false;
        let (x,y) = coords;

        for (dy,&byte) in sprite.iter().enumerate() {
            let y = (y as usize + dy) % 32;
            let byte = ((byte as u64).swap_bytes()).rotate_right(x as u32);
            self.framebuffer[y] ^= byte; 
            if self.framebuffer[y] & byte != byte {
                collision = true;
            }
        }
        
        collision
    }

    pub fn clear(&mut self) {
        for row in self.framebuffer.iter_mut() {
            *row = 0;
        }
    }
     
}

#[derive(Copy,Clone,Debug)]
pub struct Row {
    row: u64, 
    index: u8,
}


impl From<u64> for Row {
    fn from(row: u64) -> Self {
        Row {
            row,
            index: 0
        }
    }
}


impl Iterator for Row {
    type Item = bool;
    
    fn next(&mut self) -> Option<bool> {
        let bit_index = self.index;

        if bit_index > 64 {
            return None;
        }
    
        self.index += 1;
        
        match self.row & (FIRST_BIT>>bit_index) {
            0 => Some(false),
            _ => Some(true),
        }    
    }
}



#[derive(Copy,Clone,Debug)]
pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
        }
    }

    pub fn first_pressed_key(&self) -> Option<u8> {
        for (i, &key) in self.keys.iter().enumerate() {
            if key {
                return Some(i as u8);
            }
        }
        None
    }
}

