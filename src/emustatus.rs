struct Chip8Emu{
    memory: [u8; 4096], //Memory for chip8
    gpr: [u8;16],//gp registers
    ir:u16, //index register
    pc:u16,//program counter register
    sp:u8,//stack pointer
    dt:u8,//Delay timer
    st:u8,//Sound timer

}

impl Chip8Emu{ //Functions for emulator
    pub fn memorymap(){




    }

}