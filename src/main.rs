pub struct Chip8 {
    pub memory: [u8; 4096], // 4k memory
    pub v: [u8; 16],        // 16 registers
    pub i: u16,             // index register
    pub pc: u16,            // program counter
    pub gfx: [u8; 64 * 32], // graphics buffer
    pub delay_timer: u8,    // delay timer
    pub sound_timer: u8,    // sound timer
    pub stack: [u16; 16],   // stack
    pub sp: u16,            // stack pointer
    pub key: [u8; 16],      // keypad
    pub draw_flag: bool,    // draw flag
}

impl Chip8 {
    fn new() -> Chip8 {
        let mut memory = [0; 4096];
        let fontset = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        memory[..80].copy_from_slice(&fontset);
        Chip8 {
            memory,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
        }
    }
    fn emulate_cycle(&mut self) {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;

        self.pc += 2;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let kk = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0xE0 => {
                    self.gfx = [0; 64 * 32];
                    self.draw_flag = true;
                }
                0xEE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            0x1000 => self.pc = opcode & 0x0FFF,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                if self.v[x as usize] == kk {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v[x as usize] != kk {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0x6000 => self.v[x as usize] = kk,
            0x7000 => {
                let (result, _) = self.v[x as usize].overflowing_add(kk);
                self.v[x as usize] = result;
            }
            0x8000 => match opcode & 0x000F {
                0x0 => self.v[x as usize] = self.v[y as usize],
                0x1 => self.v[x as usize] |= self.v[y as usize],
                0x2 => self.v[x as usize] &= self.v[y as usize],
                0x3 => self.v[x as usize] ^= self.v[y as usize],
                0x4 => {
                    let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                    self.v[0xF] = if overflow { 0 } else { 1 };
                    self.v[x as usize] = result;
                }
                0x5 => {
                    let (result, overflow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                    self.v[0xF] = if overflow { 0 } else { 1 };
                    self.v[x as usize] = result;
                }
                0x6 => {
                    self.v[0xF] = self.v[x as usize] & 0x1;
                    self.v[x as usize] >>= 1;
                }
                0x7 => {
                    let (result, overflow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                    self.v[0xF] = if overflow { 0 } else { 1 };
                    self.v[x as usize] = result;
                }
                0xE => {
                    self.v[0xF] = self.v[x as usize] >> 7;
                    self.v[x as usize] <<= 1;
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            0x9000 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0xA000 => self.i = opcode & 0x0FFF,
            0xB000 => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            0xC000 => self.v[x as usize] = rand::random::<u8>() & kk,
            0xD000 => {
                self.v[0xF] = 0;
                for yline in 0..n {
                    let pixel = self.memory[self.i as usize + yline as usize];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            let x = self.v[x as usize] as u16 + xline as u16;
                            let y = self.v[y as usize] as u16 + yline as u16;
                            let index = (x + y * 64) as usize;

                            if x >= 64 || y >= 32 {
                                continue;
                            }
                            if self.gfx[index] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[index] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
            }

            0xE000 => match opcode & 0x00FF {
                0x9E => {
                    if self.key[self.v[x as usize] as usize] == 1 {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    if self.key[self.v[x as usize] as usize] == 0 {
                        self.pc += 2;
                    }
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },

            0xF000 => match opcode & 0x00FF {
                0x07 => self.v[x as usize] = self.delay_timer,
                0x0A => {
                    let mut key_press = false;
                    for i in 0..16 {
                        if self.key[i] == 1 {
                            self.v[x as usize] = i as u8;
                            key_press = true;
                        }
                    }
                    if !key_press {
                        self.pc -= 2;
                    }
                }
                0x15 => self.delay_timer = self.v[x as usize],
                0x18 => self.sound_timer = self.v[x as usize],
                0x1E => self.i += self.v[x as usize] as u16,
                0x29 => self.i = self.v[x as usize] as u16 * 5,
                0x33 => {
                    self.memory[self.i as usize] = self.v[x as usize] / 100;
                    self.memory[self.i as usize + 1] = (self.v[x as usize] / 10) % 10;
                    self.memory[self.i as usize + 2] = self.v[x as usize] % 10;
                }
                0x55 => {
                    for i in 0..=x {
                        self.memory[self.i as usize + i as usize] = self.v[i as usize];
                    }
                }
                0x65 => {
                    for i in 0..=x {
                        self.v[i as usize] = self.memory[self.i as usize + i as usize];
                    }
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}

fn init_canvas(context: &sdl2::Sdl) -> sdl2::render::Canvas<sdl2::video::Window> {
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", 64 * 10, 32 * 10)
        .position_centered()
        .build()
        .unwrap();
    window
        .into_canvas()
        .build()
        .expect("Failed to build canvas")
}

fn load_rom(memory: &mut [u8; 4096], rom_path: &str) {
    let rom = std::fs::read(rom_path).expect("Failed to read ROM file");
    if rom.is_empty() {
        panic!("ROM file is empty");
    }
    if rom.len() > 4096 - 512 {
        panic!("ROM file is too big");
    }
    memory[512..(rom.len() + 512)].copy_from_slice(&rom[..]);
}

fn handle_keypress(memory: &mut [u8; 16], key: sdl2::keyboard::Keycode, state: bool) {
    match key {
        sdl2::keyboard::Keycode::Num1 => memory[0x1] = state as u8,
        sdl2::keyboard::Keycode::Num2 => memory[0x2] = state as u8,
        sdl2::keyboard::Keycode::Num3 => memory[0x3] = state as u8,
        sdl2::keyboard::Keycode::Num4 => memory[0xC] = state as u8,
        sdl2::keyboard::Keycode::Q => memory[0x4] = state as u8,
        sdl2::keyboard::Keycode::W => memory[0x5] = state as u8,
        sdl2::keyboard::Keycode::E => memory[0x6] = state as u8,
        sdl2::keyboard::Keycode::R => memory[0xD] = state as u8,
        sdl2::keyboard::Keycode::A => memory[0x7] = state as u8,
        sdl2::keyboard::Keycode::S => memory[0x8] = state as u8,
        sdl2::keyboard::Keycode::D => memory[0x9] = state as u8,
        sdl2::keyboard::Keycode::F => memory[0xE] = state as u8,
        sdl2::keyboard::Keycode::Z => memory[0xA] = state as u8,
        sdl2::keyboard::Keycode::X => memory[0x0] = state as u8,
        sdl2::keyboard::Keycode::C => memory[0xB] = state as u8,
        sdl2::keyboard::Keycode::V => memory[0xF] = state as u8,
        _ => (),
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = init_canvas(&sdl_context);
    let mut chip8 = Chip8::new();
    let rom_path = std::env::args().nth(1).expect("No ROM file provided");

    load_rom(&mut chip8.memory, &rom_path);
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => std::process::exit(0),
                sdl2::event::Event::KeyDown {
                    keycode: Some(key), ..
                } => handle_keypress(&mut chip8.key, key, true),

                sdl2::event::Event::KeyUp {
                    keycode: Some(key), ..
                } => handle_keypress(&mut chip8.key, key, false),
                _ => {}
            }
        }
        chip8.emulate_cycle();
        if chip8.sound_timer > 0 {
            chip8.sound_timer -= 1;
        }
        if chip8.draw_flag {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            for y in 0..32 {
                for x in 0..64 {
                    if chip8.gfx[(y * 64 + x) as usize] == 1 {
                        canvas
                            .fill_rect(sdl2::rect::Rect::new(x * 10, y * 10, 10, 10))
                            .unwrap();
                    }
                }
            }
            canvas.present();
            chip8.draw_flag = false;
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}
