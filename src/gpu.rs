/*extern crate rand;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::VecDeque;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::cpu::CPU;

type Framebuffer = [u8; SCREEN_WIDTH * SCREEN_HEIGHT];

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub(crate) struct GPUCommand {
    id: u32,
    x: u8,
    y: u8,
    height: u8,
}

pub(crate) struct GPU {
    pub framebuffer: Arc<Mutex<Framebuffer>>,
    command_buffers: Arc<Mutex<VecDeque<GPUCommand>>>,
    barrier: Option<Arc<Barrier>>,
}

impl GPU {
    pub fn new() -> Self {
        Self {
            framebuffer: Arc::new(Mutex::new([Default::default(); SCREEN_WIDTH * SCREEN_HEIGHT])),
            command_buffers: Arc::new(Mutex::new(VecDeque::new())),
            barrier: None
        }
    }

    pub fn initialize(&mut self) -> Arc<Barrier> {
        let barrier = Arc::new(Barrier::new(1));

        self.barrier = Some(Arc::clone(&barrier));

        return barrier;
    }

    pub fn execute(&mut self,cpu:Arc<Mutex<CPU>>) {
        let commands: Vec<GPUCommand> = {
            let mut command_buffer = self.command_buffers.lock().unwrap();
            command_buffer.drain(..).collect() // Collecting maintains FIFO order
        };
        let mut framebuffer = self.framebuffer.lock().unwrap();

        commands.into_par_iter().for_each(|command | {
            let mut pixel:u8;

            cpu.lock().unwrap().registers[0xF] = 0;
            for yline in 0..command.height {
                pixel = cpu.lock().unwrap().game_memory[(cpu.lock().unwrap().index_register + yline as u16) as usize];

                for xline in 0..8 {
                    if (pixel & (0x80 >> xline)) != 0 {
                        if self.framebuffer.lock().unwrap()[(command.x + xline + ((command.y + yline) * 64)) as usize] == 1 {
                            cpu.lock().unwrap().registers[0xF] = 1;
                        }

                        self.framebuffer.lock().unwrap()[(command.x + xline + ((command.y + yline) * 64)) as usize] ^= 1;
                    }
                }
            }
        });

        self.barrier.as_ref().unwrap().wait();   
    }

    pub fn push_command(&self,command: GPUCommand) {
        self.command_buffers.lock().unwrap().push_back(command);
    }
}*/