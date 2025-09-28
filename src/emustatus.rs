use core::panic;
use std::fs;

use crate::opcodeparse::dump_rom;
use crate::opcodeparse::parser_gen;

pub struct Chip8Emu {
    pub memory: [u8; 4096],      //Memory for chip8
    pub gpr: [u8; 16],           //gp registers
    pub ir: u16,                 //index register
    pub pc: u16,                 //program counter register
    pub sp: u8,                  //stack pointer
    pub dt: u8,                  //Delay timer
    pub st: u8,                  //Sound timer
    pub stack: Vec<u16>,         //stack
    pub display: [[u8; 64]; 32], //displayvalues
    pub keypad: [bool; 16],
}

impl Chip8Emu {
    //Functions for emulator

    pub fn new() -> Self {
        Chip8Emu {
            memory: [0; 4096], //can fit about 3500 instructions in here
            gpr: [0; 16],
            ir: 0,
            pc: 0x200, //beginning of array
            sp: 0,
            dt: 0,
            st: 0,
            stack: vec![0; 16],
            display: [[0; 64]; 32],
            keypad: [false; 16],
        }
    }

    //pub fn gameopen()

    pub fn mapmem(&mut self, input_buf: Vec<u8>) { //absolutely cooks the instr. COULD be due to to aggro cut
        let start = 512;
        let end = start + input_buf.len(); 

        if end <= self.memory.len() {
            
            self.memory[start..end].copy_from_slice(&input_buf);
        }
        println!("First few bytes at 0x200: {:02X} {:02X} {:02X} {:02X}", 
                 self.memory[512], self.memory[513], self.memory[514], self.memory[515]);
    
        println!("Memory mapped");
    }
    pub fn dumpmemory(&self) {
        //Hardcoded for now
        let mut membuffer:Vec<String> = Vec::new();
        for x in 0..self.memory.len() {
            membuffer.push(format!("0x{:02x}", self.memory[x]));//print mem in base16
        }

        let contents = membuffer.join("\n");
        fs::write("memdump.txt", contents).expect("File write failed");
    }
    /* 
    pub fn displaytest(&mut self) {
        //Sets bottom display row to white pixels
        for i in 0..32 {
            self.display[63][i] = 1;
        }
    }*/

    pub fn execute(&mut self) {
        //parser_gen(self);
    }
}
