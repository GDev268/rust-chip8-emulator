use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::{opcodes::*};

pub(crate) const SCREEN_WIDTH: usize = 64;
pub(crate) const SCREEN_HEIGHT: usize = 32;

type BYTE = u8;

type Framebuffer = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];

const CHIP8_FONTSET:[u8;80] =
[ 
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
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
pub(crate) struct CPU {
    pub game_memory: [BYTE; 0xFFF], //PROGRAM RAM
    pub registers: [BYTE; 16],
    pub index_register: u16,
    pub program_counter: u16, //MEMORY POINTER
    pub stack: [u16; 16],
    pub stack_pointer: u16,
    pub cur_opcode: u16,
    pub framebuffer:Framebuffer,
    pub key: [u8;16],
    pub delay_timer: BYTE,
    pub sound_timer: BYTE
}

impl CPU {
    pub fn new() -> Self {
        Self {
            game_memory: [Default::default(); 0xFFF],
            registers: [Default::default(); 16],
            index_register: 0x000000,
            program_counter: 0,
            stack: [Default::default(); 16],
            stack_pointer: 0x000000,
            cur_opcode: 0x000000,
            framebuffer: [Default::default(); SCREEN_WIDTH * SCREEN_HEIGHT],
            key: [Default::default(); 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.index_register = 0;
        self.program_counter = 0x200;
        self.index_register = 0;
        self.stack_pointer = 0;
        self.cur_opcode = 0;
        self.stack = [Default::default(); 16];
        self.registers = unsafe { std::mem::zeroed() };

        for i in 0..CHIP8_FONTSET.len() {
            self.game_memory[i] = CHIP8_FONTSET[i];
        }
    }

    pub fn load_rom(&mut self, path: String) {
        let mut file: File = File::open(path).expect("Failed to open the file!");

        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)
            .expect("Failed to read the file!");

        let start = 0x200 as usize;
        let end = 0x200 + buffer.len() as usize;

        self.game_memory[start..end].copy_from_slice(&buffer)
    }

    pub fn get_next_opcode(&mut self) {
        self.cur_opcode = self.game_memory[self.program_counter as usize] as u16;

        self.cur_opcode =
            self.cur_opcode << 8 | self.game_memory[(self.program_counter + 1) as usize] as u16;

        //self.program_counter += 2;

        //println!("\nOpcode Info: ");
        //println!("{:?}", self.cur_opcode);
        //println!("0x{:X}", self.cur_opcode);
        //println!("{:016b}", self.cur_opcode);
    }

    pub fn get_delay(&self) -> BYTE {
        return self.delay_timer;
    }

    pub fn update(&mut self) {
        self.get_next_opcode();

        match self.cur_opcode & 0xF000 {
            //Gets the first value of an opcode (Ex: From: 0x1234 gets 1);
            0x0000 => match self.cur_opcode & 0x000F {
                0x0000 => {
                    self.framebuffer = [Default::default(); SCREEN_WIDTH * SCREEN_HEIGHT];
                    self.program_counter += 2
                },
                0x000E => {
                    opcode_0_0ee(self);
                    self.program_counter += 2;
                },
                _ => {}
            },
            0x1000 => opcode_1_nnn(self, self.cur_opcode),
            0x2000 => opcode_2_nnn(self, self.cur_opcode),
            0x3000 => opcode_3_xnn(self, self.cur_opcode),
            0x4000 => opcode_4_xnn(self, self.cur_opcode),
            0x5000 => opcode_5_xy0(self, self.cur_opcode),
            0x6000 => {
                opcode_6_xnn(self, self.cur_opcode);
                self.program_counter += 2;
            }
            0x7000 => {
                opcode_7_xnn(self, self.cur_opcode);
                self.program_counter += 2;
            }
            0x8000 => {
                match self.cur_opcode & 0x000F {
                    0x0000 => {
                        opcode_8_xy0(self, self.cur_opcode);
                    }
                    0x0001 => {
                        opcode_8_xy1(self, self.cur_opcode);
                    }
                    0x0002 => {
                        opcode_8_xy2(self, self.cur_opcode);
                    }
                    0x0003 => {
                        opcode_8_xy3(self, self.cur_opcode);
                    }
                    0x0004 => {
                        opcode_8_xy4(self, self.cur_opcode);
                    }
                    0x0005 => {
                        opcode_8_xy5(self, self.cur_opcode);
                    }
                    0x0006 => {
                        opcode_8_xy6(self, self.cur_opcode);
                    }
                    0x0007 => {
                        opcode_8_xy7(self, self.cur_opcode);
                    }
                    0x000E => {
                        opcode_8_xye(self, self.cur_opcode);
                    }
                    _ => {}
                }

                self.program_counter += 2;
            }
            0x9000 => opcode_9_xy0(self, self.cur_opcode),
            0xA000 => {
                opcode_a_nnn(self, self.cur_opcode);
                self.program_counter += 2;
            }
            0xB000 => {
                opcode_b_nnn(self, self.cur_opcode);
                self.program_counter += 2;
            }
            0xC000 => {
                opcode_c_xnn(self, self.cur_opcode);
                self.program_counter += 2;
            }
           0xD000 => {
            opcode_d_xyn(self, self.cur_opcode);
            self.program_counter += 2;

            }
            0xE000 => match self.cur_opcode & 0x00FF {
                0x009E => {opcode_e_x9e(self, self.cur_opcode)},
                0x00A1 => {opcode_e_xa1(self, self.cur_opcode)},
                _ => {}
            }
            0xF000 => match self.cur_opcode & 0x00FF {
                0x0007 => {
                    opcode_f_x07(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x000A => {
                    opcode_f_x0a(self, self.cur_opcode);
                },
                0x0015 => {
                    opcode_f_x15(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x0018 => {
                    opcode_f_x18(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x001E => {
                    opcode_f_x1e(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x0029 => {
                    opcode_f_x29(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x0033 => {
                    opcode_f_x33(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x0055 => {
                    opcode_f_x55(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                0x0065 => {
                    opcode_f_x65(self, self.cur_opcode);
                    self.program_counter += 2;
                },
                _ => {}
            }
            _ => {}
        }

       /* println!("\rProgram Info: \nPC: {:?} \nOPCODE: 0x{:X} \nI: 0x{:X} \nSP: 0x{:X} \nREG: {:?} \nSTACK: {:?}",
        self.program_counter, self.cur_opcode, self.index_register, self.stack_pointer,
       self.registers, self.stack); */


        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            
            self.sound_timer -= 1;
        }

    }
}
