use core::panic;

use crate::opcodeparse::dump_rom;
use crate::opcodeparse::parser_gen;

pub struct Chip8Emu{
    pub memory: [u8; 4096], //Memory for chip8
    pub gpr: [u8;16],//gp registers
    pub ir:u16, //index register
    pub pc:u16,//program counter register
    pub sp:u8,//stack pointer
    dt:u8,//Delay timer
    st:u8,//Sound timer
    pub stack:Vec<u16>,//stack
    pub display: [[u8; 32]; 64],//displayvalues
}

impl Chip8Emu{ //Functions for emulator


    pub fn new() -> Self{
        Chip8Emu{
            memory: [0; 4096],//can fit about 3500 instructions in here
            gpr: [0;16],
            ir: 0,
            pc: 0x200, //beginning of array
            sp: 0,
            dt: 0,
            st: 0,
            stack: Vec::new(),
            display: [[0;32];64],
        }


    }

    pub fn dumpmemory(&self){ //Hardcoded for now
        for x in 0..self.memory.len(){
            println!("0x{:02x}",self.memory[x]); //print mem in base16
        }
    }


    pub fn openself(&mut self) {

        let val = dump_rom();

        match val{ //don't propogate,just handle here
            Ok(ok) => {
                //println!("Vector is {:?}",ok);
                println!("First entry is 0x{:02x}",ok[0]); //map the data to SOMETHING then address it

                for x in 0..ok.len() {
                    self.memory[512 + x] = ok[x];
                }
            },
            Err(e) => panic!("Error is {e}"),
        }
    }

    pub fn displaytest(&mut self){ //Sets bottom display row to white pixels
        for i in 0..32 {
            self.display[63][i] = 1;
        }
        
    }


    pub fn execute(&mut self){

        parser_gen( self);

    }

}