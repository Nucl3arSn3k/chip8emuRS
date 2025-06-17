pub struct Chip8Emu{
    memory: [u8; 4096], //Memory for chip8
    gpr: [u8;16],//gp registers
    ir:u16, //index register
    pc:u16,//program counter register
    sp:u8,//stack pointer
    dt:u8,//Delay timer
    st:u8,//Sound timer
    stack:Vec<u16>,//stack
    display: [[u8; 32]; 64],//displayvalues
}

impl Chip8Emu{ //Functions for emulator


    pub fn new() -> Self{
        Chip8Emu{
            memory: [0; 4096],
            gpr: [0;16],
            ir: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            stack: Vec::new(),
            display: [[0;32];64],
        }


    }

    pub fn memorymap(&self){ //Hardcoded for now




    }


    pub fn displaytest(){}

}