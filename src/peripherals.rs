use std::option::Option;

#[derive(Copy,Clone,Debug)]
pub struct Framebuffer {
    framebuffer: [u64; 32]
}


impl Framebuffer {
    pub fn new() -> Self {
        Framebuffer {
            framebuffer: [0; 32],
        }
    }

    pub fn rows(&self) -> &[Row] {
        let mut rows : [Row; 32];
        for (i, &row) in self.framebuffer.iter().enumerate() {
            rows[i] = row.into();
        }
        &rows[..]
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        self.framebuffer[y as usize] & (1<<x) != 0 
    }

    pub fn draw(&mut self, coords: (u8,u8) , sprite: &[u8]) -> bool {
        let mut collision = false;
        let (x,y) = coords;

        for (dy,&byte) in sprite.iter().enumerate() {
            let y = (y as usize + dy) % 32;
            let byte = (byte as u64).rotate_left(x as u32);
            self.framebuffer[y] ^= byte; 
            if self.framebuffer[y] & byte != byte {
                collision = true;
            }
        }
        
        collision
    }
     
}

#[derive(Copy,Clone,Debug)]
struct Row {
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
        
        match self.row & (1<<bit_index) {
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
}

