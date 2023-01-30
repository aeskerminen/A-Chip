use crate::memory::{Memory, allocate_memory};
use rand::Rng;

const WIDTH : usize = 64;
const HEIGHT : usize = 32;
const DISPLAY_SIZE : usize = WIDTH * HEIGHT;

pub struct CPU {
    pub screen: [[bool; WIDTH]; HEIGHT],

    memory: Memory,

    register: [u8;16],
    i: usize,
    pc: usize,
  
    stack: [usize;64],
    sp: usize,

    pub keypad: [u8; 16],

    dt: u8,
    st: u8,

    draw_flag: bool
}

impl CPU {
    pub fn new() -> CPU {
        let register = [0;16];
        let stack = [0;64];
        let memory = allocate_memory();
        
        CPU {
            screen: [[false; WIDTH]; HEIGHT],
            memory,
            register,
            i: 0,
            pc: 0x200,
            stack,
            sp: 0,
            keypad: [0; 16],
            dt: 0,
            st: 0,
            draw_flag: false
        }
    }

    pub fn load_buffer_to_memory(&mut self, buffer: std::vec::Vec<u8>) {
        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
    }

    pub fn set_keypad(&mut self, value: u8, index: usize) {
        self.keypad[index] = value;
    }

    pub fn set_draw(&mut self, b: bool) {
        self.draw_flag = b;
    }

    pub fn should_draw(&self) -> bool {
        self.draw_flag
    }

    fn fetch_opcode(&mut self) -> u16 {
        let high_order = self.memory[self.pc];
        let low_order = self.memory[(self.pc + 1)];
    
        let opcode : u16 = (high_order as u16) << 8 | low_order as u16;
    
        return opcode;
    }
    
    pub fn tick_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch_opcode();
        self.pc += 2;
        
        match op & 0xF000 {
            0x0000 => {match op & 0x000F {
                0x0000 => {println!("{:#02x}, fn: op_00e0", op); self.op_00e0(op)},
                0x000E => {println!("{:#02x}, fn: op_00ee", op); self.op_00ee(op)},
                _ => println!("Not implemented... {:#02x}", op)
            }},
            0x1000 => {println!("{:#02x}, fn: 1nnn", op); self.op_1nnn(op)},
            0x2000 => {println!("{:#02x}, fn: 2nnn", op); self.op_2nnn(op)},
            0x3000 => {println!("{:#02x}, fn: 3xnn", op); self.op_3xnn(op)},
            0x4000 => {println!("{:#02x}, fn: 4xnn", op); self.op_4xnn(op)},
            0x5000 => {println!("{:#02x}, fn: 5xy0", op); self.op_5xy0(op)},
            0x6000 => {println!("{:#02x}, fn: 6xnn", op); self.op_6xnn(op)},
            0x7000 => {println!("{:#02x}, fn: 7xnn", op); self.op_7xnn(op)},
            0x8000 => {
                match op & 0x000F {
                    0x0000 => {println!("{:#02x}, fn: op_8xy0", op); self.op_8xy0(op)},
                    0x0001 => {println!("{:#02x}, fn: op_8xy1", op); self.op_8xy1(op)},
                    0x0002 => {println!("{:#02x}, fn: op_8xy2", op); self.op_8xy2(op)},
                    0x0003 => {println!("{:#02x}, fn: op_8xy3", op); self.op_8xy3(op)},
                    0x0004 => {println!("{:#02x}, fn: op_8xy4", op); self.op_8xy4(op)},
                    0x0005 => {println!("{:#02x}, fn: op_8xy5", op); self.op_8xy5(op)},
                    0x0006 => {println!("{:#02x}, fn: op_8xy6", op); self.op_8xy6(op)},
                    0x0007 => {println!("{:#02x}, fn: op_8xy7", op); self.op_8xy7(op)},
                    0x000e => {println!("{:#02x}, fn: op_8xye", op); self.op_8xye(op)},
                    _ => println!("Not implemented... {:#02x}", op)
                }
            },
            0x9000 => {println!("{:#02x}, fn: op_9xy0", op); self.op_9xy0(op)},
            0xA000 => {println!("{:#02x}, fn: op_annn", op); self.op_annn(op)},
            0xB000 => {println!("{:#02x}, fn: op_bnnn", op); self.op_bnnn(op)},
            0xC000 => {println!("{:#02x}, fn: op_cxnn", op); self.op_cxnn(op)},
            0xD000 => {println!("{:#02x}, fn: op_dxyn", op); self.op_dxyn(op)},
            0xE000 => {
                match op & 0x000F {
                    0x000E => {println!("{:#02x}, fn: op_ex9e", op); self.op_ex9e(op)},
                    0x0001 => {println!("{:#02x}, fn: op_exa1", op); self.op_exa1(op)},
                    _ => println!("Not implemented... {:#02x}", op)
                }
            },
            0xF000 => {
                match op & 0x00FF {
                    0x0007 => {println!("{:#02x}, fn: op_fx07", op); self.op_fx07(op)},
                    0x000A => {println!("{:#02x}, fn: op_fx0a", op); self.op_fx0a(op)},
                    0x0015 => {println!("{:#02x}, fn: op_fx15", op); self.op_fx15(op)},
                    0x0018 => {println!("{:#02x}, fn: op_fx18", op); self.op_fx18(op)},
                    0x001E => {println!("{:#02x}, fn: op_fx1e", op); self.op_fx1e(op)},
                    0x0029 => {println!("{:#02x}, fn: op_fx29", op); self.op_fx29(op)},
                    0x0033 => {println!("{:#02x}, fn: op_fx33", op); self.op_fx33(op)},
                    0x0055 => {println!("{:#02x}, fn: op_fx55", op); self.op_fx55(op)},
                    0x0065 => {println!("{:#02x}, fn: op_fx65", op); self.op_fx65(op)},
                    _ => println!("Not implemented... {:#02x}", op)
                }
            }
            _ => println!("Not implemented... {:#02x}", op)
        }
    }

    fn op_00e0(&mut self, op: u16) {
        self.screen =[[false; WIDTH]; HEIGHT];
        self.draw_flag = true;
    }
    
    fn op_00ee(&mut self, op: u16) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }
    
    fn op_1nnn(&mut self, op: u16) {
        self.pc = get_nnn(op) as usize;
    }
    
    fn op_2nnn(&mut self, op: u16) {
        self.stack[self.sp] = self.pc;
        self.sp += 1; 
    
        self.pc = get_nnn(op) as usize;
    }
    
    fn op_3xnn(&mut self, op: u16) {
        if self.register[get_2(op) as usize] == get_nn(op) {
            self.pc += 2;
        }
    }
    
    fn op_4xnn(&mut self, op: u16) {
        if self.register[get_2(op) as usize] != get_nn(op) {
            self.pc += 2;
        }
    }
    
    fn op_5xy0(&mut self, op: u16) {
        if self.register[get_2(op) as usize] == self.register[get_3(op) as usize] {
            self.pc += 2;
        }
    }
    
    fn op_6xnn(&mut self, op: u16) {
        self.register[get_2(op) as usize] = get_nn(op);
    }

    fn op_7xnn(&mut self, op: u16) {
        self.register[get_2(op) as usize] = ((self.register[get_2(op) as usize] as u16 + get_nn(op) as u16) & 0x00FF) as u8;
    }
    
    fn op_8xy0(&mut self, op: u16) {
        self.register[get_2(op) as usize] = self.register[get_3(op) as usize];
    }
    
    fn op_8xy1(&mut self, op: u16) {
        self.register[get_2(op) as usize] |= self.register[get_3(op) as usize];
    }
    
    fn op_8xy2(&mut self, op: u16) {
        self.register[get_2(op) as usize] &= self.register[get_3(op) as usize];
    }
    
    fn op_8xy3(&mut self, op: u16) {
        self.register[get_2(op) as usize] ^= self.register[get_3(op) as usize];
    }
    
    fn op_8xy4(&mut self, op: u16) {
        let add = self.register[get_2(op) as usize] as u16 + self.register[get_3(op) as usize] as u16;
        
        self.register[get_2(op) as usize] = (add & 0x00FF) as u8;

        self.register[0xF] = (add >> 8) as u8;
    }
    
    fn op_8xy5(&mut self, op: u16) {
        let add = self.register[get_2(op) as usize] as i16 - self.register[get_3(op) as usize] as i16;
        
        self.register[get_2(op) as usize] = (add & 0x00FF) as u8;

        self.register[0xF] = ((add >> 8) + 1) as u8;
    }
    
    fn op_8xy6(&mut self, op: u16) {
        let xr : u8 = self.register[get_2(op) as usize];
    
        self.register[0xF] = xr & 0x01;
        
        self.register[get_2(op) as usize] = xr >> 1;
    }   
    
    fn op_8xy7(&mut self, op: u16) {
        let add = self.register[get_2(op) as usize] as i16 - self.register[get_3(op) as usize] as i16;

        self.register[get_2(op) as usize] = (add & 0x00FF) as u8;
        self.register[0xF] = ((add >> 8) + 1) as u8;
    }
    
    fn op_8xye(&mut self, op: u16) {
        let xr : u8 = self.register[get_2(op) as usize];
    
        self.register[0xF] = ((xr & 0x80) >> 7) as u8;
        
        self.register[get_2(op) as usize] = xr << 1;
    }
    
    fn op_9xy0(&mut self, op: u16) {
        let xr : u8 = self.register[get_2(op) as usize];
        let yr : u8 = self.register[get_3(op) as usize];
    
        if xr != yr {
            self.pc += 2;
        }
    }
    
    fn op_annn(&mut self, op: u16) {
        self.i = get_nnn(op) as usize;
    }
    
    fn op_bnnn(&mut self, op: u16) {
        self.pc = (self.register[0] + get_nnn(op) as u8) as usize;
    }
    
    fn op_cxnn(&mut self, op: u16) {
        let r = rand::thread_rng().gen_range(0..255);
        self.register[get_2(op) as usize] = r & (get_nn(op));
    }
    
    fn op_dxyn(&mut self, op: u16) {
        self.draw(op);
    }
    
    // input
    fn op_ex9e(&mut self, op: u16) {
        if self.keypad[self.register[get_2(op) as usize] as usize] != 0 {
            self.pc += 2;
        } 
    }
    
    // input
    fn op_exa1(&mut self, op: u16) {
        if self.keypad[self.register[get_2(op) as usize] as usize] != 1 {
            self.pc += 2;
        } 
    }   
    
    fn op_fx07(&mut self, op: u16) {
        self.register[get_2(op) as usize] = self.dt;
    }
    
    // input
    fn op_fx0a(&mut self, op: u16) {
        for i in 0..0xF {
            if self.keypad[i] != 0 {
                self.register[get_2(op) as usize] = i as u8;
                self.pc -= 2;
                break;
            }
        }
    }
    
    fn op_fx15(&mut self, op: u16) {
        self.dt = self.register[get_2(op) as usize];
    }
    
    fn op_fx18(&mut self, op: u16) {
        self.st = self.register[get_2(op) as usize];
    }
    
    fn op_fx1e(&mut self, op: u16) {
        self.i += self.register[get_2(op) as usize] as usize;
    }
    
    fn op_fx29(&mut self, op: u16) {
        self.i = ((5 * self.register[get_2(op) as usize])) as usize;
    }
    
    fn op_fx33(&mut self, op: u16) {
        let dec = self.register[get_2(op) as usize];

        self.memory[self.i + 0] = dec / 100;
        self.memory[(self.i + 1)] = dec % 100 / 10;
        self.memory[(self.i) + 2] = dec % 10;
    }
    
    fn op_fx55(&mut self, op: u16) {
        for x in 0..(get_2(op) + 1) {
            self.memory[x as usize + self.i] = self.register[x as usize];
        }
    }
    
    fn op_fx65(&mut self, op: u16) {
        for x in 0..(get_2(op) + 1) {
            self.register[x as usize] = self.memory[(self.i + x as usize)];
        }
    }

    pub fn draw(&mut self, op : u16) {
        let n = (op & 0x000F) as usize; // Sprite height in bytes to be displayed;
        let w = 8 as usize; // Sprites always 8 or 16 pixels
        let h = n  as usize; // Height can be 0 to 16 pixels

        // Sprite X & Y coordinates fetched from our V register
        let x_coord = usize::from(self.register[((op & 0x0F00) >> 8) as usize]);
        let y_coord = usize::from(self.register[((op & 0x00F0) >> 4) as usize]);

        self.register[0xF] = 0; // The collision flag must be off before rendering

        for yline in 0..h {
            // Fetch low & high bytes from memory if in extended (16x16 mode)
            let pixel: u16 = self.memory[(self.i as usize + yline as usize)] as u16;

            for xline in 0..w {
                let x = (x_coord + xline) % WIDTH;
                let y = (y_coord + yline) % HEIGHT;

                // Evaluate our sprite by ANDing the value and shifting right by one (bit 7 -> 0).
                // The same applies for extended: bitmask is extended to cover 16 bits.
                if pixel & 0x80  >> xline != 0 {
                    self.register[0xF] |= self.screen[y % HEIGHT][x % WIDTH] as u8;
                    self.screen[y][x] ^= true;
                }
            }
        }
        // Upscale in Normal mode
        
        self.draw_flag = true;
    }
}

fn get_1(op: u16) -> u8 {
    return ((op & 0xF000) >> 12) as u8;
}

fn get_2(op: u16) -> u8 {
    return ((op & 0x0F00) >> 8) as u8;
}

fn get_3(op: u16) -> u8 {
    return ((op & 0x00F0) >> 4) as u8;
}

fn get_4(op: u16) -> u8 {
    return (op & 0x000F) as u8;
}

fn get_nn(op: u16) -> u8 {
    return (op & 0x00FF) as u8;
}

fn get_nnn(op: u16) -> u16 {
    return (op & 0x0FFF) as u16;
}


