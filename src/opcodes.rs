use std::{arch::x86_64, f32::RADIX};

use rand::Rng;

use crate::cpu::{CPU, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn opcode_0_0ee(cpu: &mut CPU) {
    cpu.stack_pointer -= 1;
    cpu.program_counter = cpu.stack[cpu.stack_pointer as usize] as u16;
    cpu.program_counter += 2;
}

pub fn opcode_1_nnn(cpu: &mut CPU, opcode: u16) {
    cpu.program_counter = opcode & 0x0FFF;
}

pub fn opcode_2_nnn(cpu: &mut CPU, opcode: u16) {
    cpu.stack[cpu.stack_pointer as usize] = cpu.program_counter;
    cpu.stack_pointer += 1;
    cpu.program_counter = opcode & 0x0FFF;
}

pub fn opcode_3_xnn(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    if cpu.registers[regx as usize] == (opcode & 0x00FF) as u8 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn opcode_4_xnn(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    if cpu.registers[regx as usize] != (opcode & 0x00FF) as u8 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn opcode_5_xy0(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00; //Ex: 0x200
    regx >>= 8; // Ex: 0x2

    let mut regy = opcode & 0x00F0; //Ex: 0x20
    regy >>= 4; // Ex: 0x2

    if cpu.registers[regx as usize] == cpu.registers[regy as usize] {
        cpu.program_counter += 2;
    }
}

pub fn opcode_6_xnn(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.registers[regx as usize] = (opcode & 0x00FF) as u8;
}

pub fn opcode_7_xnn(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.registers[regx as usize] =
        cpu.registers[regx as usize].wrapping_add((opcode & 0x00FF) as u8);
}

pub fn opcode_8_xy0(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let yval = cpu.registers[regy as usize];

    cpu.registers[regx as usize] = yval;
}

pub fn opcode_8_xy1(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let yval = cpu.registers[regy as usize];

    cpu.registers[regx as usize] |= yval;
}

pub fn opcode_8_xy2(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let yval = cpu.registers[regy as usize];

    cpu.registers[regx as usize] &= yval;
}

pub fn opcode_8_xy3(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let yval = cpu.registers[regy as usize];

    cpu.registers[regx as usize] ^= yval;
}

pub fn opcode_8_xy4(cpu: &mut CPU, opcode: u16) {
    cpu.registers[0xF] = 0;

    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let xval = cpu.registers[regx as usize];
    let yval = cpu.registers[regy as usize];

    if yval > xval {
        cpu.registers[0xF] = 1;
    }

    cpu.registers[regx as usize] += yval;
}

pub fn opcode_8_xy5(cpu: &mut CPU, opcode: u16) {
    cpu.registers[0xF] = 1;

    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let xval = cpu.registers[regx as usize];
    let yval = cpu.registers[regy as usize];

    if yval > xval {
        cpu.registers[0xF] = 0;
    }

    cpu.registers[regx as usize] -= yval;
}

pub fn opcode_8_xy6(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    cpu.registers[regx as usize] = cpu.registers[regy as usize];
    cpu.registers[regx as usize] >>= 1;
    cpu.registers[0xF] = cpu.registers[regy as usize] & 0x0001;
}

pub fn opcode_8_xy7(cpu: &mut CPU, opcode: u16) {
    cpu.registers[0xF] = 1;

    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let xval = cpu.registers[regx as usize];
    let yval = cpu.registers[regy as usize];

    if yval > xval {
        cpu.registers[0xF] = 0;
    }

    cpu.registers[regx as usize] = yval - xval;
}

pub fn opcode_8_xye(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    cpu.registers[regx as usize] = cpu.registers[regy as usize];
    cpu.registers[regx as usize] <<= 1;
    cpu.registers[0xF] = cpu.registers[regy as usize] >> 7;
}

pub fn opcode_9_xy0(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    if cpu.registers[regx as usize] != cpu.registers[regy as usize] {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn opcode_a_nnn(cpu: &mut CPU, opcode: u16) {
    cpu.index_register = opcode & 0x0FFF;
}

pub fn opcode_b_nnn(cpu: &mut CPU, opcode: u16) {
    cpu.program_counter = (opcode & 0x0FFF) + cpu.registers[0] as u16;
}

pub fn opcode_c_xnn(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let nn: u16 = opcode & 0x00FF;
    let rng: u8 = rand::thread_rng().gen();

    cpu.registers[regx as usize] = rng & nn as u8;
}

pub fn opcode_d_xyn(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut regy = opcode & 0x00F0;
    regy >>= 4;

    let x = cpu.registers[regx as usize];
    let y = cpu.registers[regy as usize];
    let height = (opcode & 0x000F) as u8;

    let mut pixel: u8;

    let mut flipped = false;
    // Iterate over each row of our sprite
    for y_line in 0..height {
        // Determine which memory address our row's data is stored
        let addr = cpu.index_register + y_line as u16;
        let pixels = cpu.game_memory[addr as usize];
        // Iterate over each column in our row
        for x_line in 0..8 {
            // Use a mask to fetch current pixel's bit. Only flip if a 1
            if (pixels & (0b1000_0000 >> x_line)) != 0 {
                // Sprites should wrap around screen, so apply modulo
                let x = (x + x_line) as usize % SCREEN_WIDTH;
                let y = (y + y_line) as usize % SCREEN_HEIGHT;

                // Get our pixel's index in the 1D screen array
                let idx = x + SCREEN_WIDTH * y;
                // Check if we're about to flip the pixel and set
                flipped |= cpu.framebuffer[idx];
                cpu.framebuffer[idx] ^= true;
            }
        }
    }
    // Populate VF register
    if flipped {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
}

pub fn opcode_e_x9e(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    if cpu.key[cpu.registers[regx as usize] as usize] == 1 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn opcode_e_xa1(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    if cpu.key[cpu.registers[regx as usize] as usize] == 0 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn opcode_f_x07(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.registers[regx as usize] = cpu.get_delay();
}

pub fn opcode_f_x0a(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let mut key_pressed: bool = false;

    for i in 0..cpu.key.len() {
        if cpu.key[i] != 0 {
            cpu.registers[regx as usize] = i as u8;
            key_pressed = true;
        }
    }

    if key_pressed {
        cpu.program_counter += 2
    };
}

pub fn opcode_f_x15(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.delay_timer = cpu.registers[regx as usize];
}

pub fn opcode_f_x18(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.sound_timer = cpu.registers[regx as usize];
}

pub fn opcode_f_x1e(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.index_register += cpu.registers[regx as usize] as u16 % 0xFFF;
}

pub fn opcode_f_x29(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    cpu.index_register = cpu.registers[regx as usize] as u16 % 0x5;
}

pub fn opcode_f_x33(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    let value = cpu.registers[regx as usize];

    let hundreds = value / 100;
    let tens = (value / 10) % 10;
    let units = value % 10;

    cpu.game_memory[(cpu.index_register) as usize] = hundreds;
    cpu.game_memory[(cpu.index_register + 1) as usize] = tens;
    cpu.game_memory[(cpu.index_register + 2) as usize] = units;
}

pub fn opcode_f_x55(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    for i in 0..regx {
        cpu.index_register += 1;
        cpu.game_memory[cpu.index_register as usize] = cpu.registers[i as usize];
    }
}

pub fn opcode_f_x65(cpu: &mut CPU, opcode: u16) {
    let mut regx = opcode & 0x0F00;
    regx >>= 8;

    for i in 0..regx {
        cpu.index_register += 1;
        cpu.registers[i as usize] = cpu.game_memory[cpu.index_register as usize];
    }
}
