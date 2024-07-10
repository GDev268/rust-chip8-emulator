mod cpu;
mod gpu;
mod opcodes;

use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};

use cpu::{CPU, SCREEN_HEIGHT, SCREEN_WIDTH};
use crossterm::{terminal::ClearType, Command};


fn main() {   
    let mut cpu = CPU::new();
    cpu.initialize();
    cpu.load_rom("./ROMS/INVADERS.ch8".to_owned());

    //println!("\n{:?}",&cpu.game_memory);
    let mut timer: f64 = 0.0;

    loop {
        timer += 0.01;
        cpu.update();

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if cpu.framebuffer[y * SCREEN_WIDTH + x] {
                    print!("█"); // Character representing set pixel
                } else {
                    print!(" "); // Character representing clear pixel
                }
            }
            println!();
        }
        
        //println!("\rCur Program Counter: {:?} | 0x{:X}",cpu.program_counter,cpu.cur_opcode);
        sleep(Duration::from_millis(25));
        crossterm::terminal::Clear(ClearType::All).execute_winapi().unwrap();

    }
    
}