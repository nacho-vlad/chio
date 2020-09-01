use pixels::{Pixels, SurfaceTexture};
use std::collections::HashMap;
use winit_input_helper::WinitInputHelper;
use winit::{
    event::{ 
        VirtualKeyCode,
    },
    event_loop::{
        ControlFlow, 
        EventLoop,
    },
    window::{
        Window,
        WindowBuilder,
    },
};
use serde::{Serialize,Deserialize};
use chio::Chip8;

struct Emulator {
    chip8: Chip8,
    pixels: Pixels<Window>,

    settings: EmuSettings,
    
    window: Window,
    event_loop: Option<EventLoop<()>>,
    input: WinitInputHelper,
}



#[derive(Serialize,Deserialize)]
struct EmuSettings {
    pub background_color : [u8;3],
    pub foreground_color : [u8;3],
    pub clock_speed : u32,
    pub key_map : HashMap<VirtualKeyCode, u8>,
}

impl Default for EmuSettings {
    fn default() -> EmuSettings {
        EmuSettings {
            background_color : [0x00,0x00,0x00],
            foreground_color : [0xFF,0xFF,0xFF],
            clock_speed: 600,
            key_map : 
                [(VirtualKeyCode::Key1 , 0),
                (VirtualKeyCode::Key2 , 1),
                (VirtualKeyCode::Key3 , 2),
                (VirtualKeyCode::Key4 , 3),
                (VirtualKeyCode::Q , 4),
                (VirtualKeyCode::W , 5),
                (VirtualKeyCode::E , 6),
                (VirtualKeyCode::R , 7),
                (VirtualKeyCode::A , 8),
                (VirtualKeyCode::S , 9),
                (VirtualKeyCode::D , 10),
                (VirtualKeyCode::F , 11),
                (VirtualKeyCode::Z , 12),
                (VirtualKeyCode::X , 13),
                (VirtualKeyCode::C , 14),
                (VirtualKeyCode::V , 15)].iter().cloned().collect(),

        }
    }

}


impl Emulator {
    fn new() -> Emulator {
        let chip8 = Chip8::new();

        let settings : EmuSettings= match std::fs::read_to_string("./settings.json") {
            Ok(s) => serde_json::from_str(&s).unwrap(),
            Err(_) => EmuSettings::default(),
        };
            
        
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Chip8")
            .build(&event_loop)
            .unwrap();

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(64,32, surface_texture).unwrap()
        };

        let input = WinitInputHelper::new();

        Emulator {
            chip8,
            pixels,

            settings,
            window,
            event_loop: Some(event_loop),

            input,
        }

    }


    fn run<P: AsRef<std::path::Path>>(mut self, path_to_rom: P) {
        
        self.chip8.load_file(path_to_rom).unwrap();

        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            
            if self.input.update(&event) {
                
                
                if self.input.key_released(VirtualKeyCode::Escape) || self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Some(size) = self.input.window_resized() {
                    self.pixels.resize(size.width, size.height);
                }
                
                self.chip8.decrement_timers();
                self.handle_inputs();
                for _ in 0..self.settings.clock_speed / 60 {
                    self.chip8.execute_cycle();
                }
                self.update_pixels();

                self.window.request_redraw();
                self.pixels.render().unwrap()
            }
        });
    }

    fn handle_inputs(&mut self) {
        
        for i in 0..16 {
            self.chip8.keypad.keys[i as usize] = false;
        }

        for (key, mapping) in self.settings.key_map.iter()  {
            if self.input.key_held(*key) {
                self.chip8.keypad.keys[*mapping as usize] = true;
            }
        }
    }

    fn update_pixels(&mut self) {
        let frame = self.pixels.get_frame();    


        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % 64;
            let y = i / 64;

            match self.chip8.framebuffer.get(x as u8,y as u8) {
                true => {
                    pixel[0] = self.settings.foreground_color[0];
                    pixel[1] = self.settings.foreground_color[1];
                    pixel[2] = self.settings.foreground_color[2];
                    pixel[3] = 0xFF;
                },
                false => {
                    pixel[0] = self.settings.background_color[0];
                    pixel[1] = self.settings.background_color[1];
                    pixel[2] = self.settings.background_color[2];
                    pixel[3] = 0xFF;
                }
            }
            
        }

    }


}

fn main() {
    let emulator = Emulator::new();
    emulator.run("./roms/TICTAC");


}
