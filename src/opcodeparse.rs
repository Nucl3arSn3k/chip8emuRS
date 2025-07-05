use crate::emustatus::{self, Chip8Emu};
use std::fs::File;
use std::io::{Read, Result};
pub fn dump_rom() -> Result<Vec<u8>> {
    let rom_path = std::env::current_dir()?.join("Lunar Lander (Udo Pernisz, 1979).ch8");
    println!("{:?}", rom_path);
    let mut state_vec: Vec<u8> = Vec::new();
    let mut game = File::open(&rom_path)?;
    game.read_to_end(&mut state_vec)?;

    Ok(state_vec)
}

pub fn parser_gen(emu_state: &mut Chip8Emu) {
    for z in 512..emu_state.memory.len() {
        let f_p = emu_state.memory[z] as u16; //First byte
        let s_p = emu_state.memory[z + 1] as u16;  //Second byte

        let opcode: u16 = (f_p << 8) | (s_p); //combine into the full instruction

        match opcode >> 12 { //extract highest nibble
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        //Clear display
                        for x in 0..64 {
                            for i in 0..32 {
                                emu_state.display[x][i] = 0;
                            }
                        }
                    }
                    0x00EE => {
                        //Return from subroutine
                        emu_state.sp -= 1;

                        emu_state.pc = emu_state.stack[emu_state.sp as usize];
                    }

                    _ => {
                        //Call a jump
                    }
                }
            }

            0x1000 => {
                match opcode {
                    _ => {
                        let addr = opcode & 0x0FFF;

                        emu_state.pc = addr; //Set program counter to address
                    }
                }
            }

            0x2000 => {
                match opcode {
                    _ => {
                        let addr = opcode & 0x0FFF;
                        
                        emu_state.stack[emu_state.sp as usize] = emu_state.pc; //current pc top of stack
                        emu_state.sp += 1; //stack pointer incremented
                        emu_state.pc = addr;
                    }
                }
            }

            0x3000 => {
                //Fix
                match opcode {
                    _ => {
                        let val = (opcode & 0x00FF) as u8;
                        let reg_index = ((opcode & 0x0F00) >> 8) as usize;
                        if emu_state.gpr[reg_index] == val {
                            emu_state.pc += 2;
                        }
                    }
                }
            }

            0x4000 => {
                match opcode {
                    _ => {
                        let val = (opcode & 0x00FF) as u8; //kk
                        let reg_index = ((opcode & 0x0F00) >> 8) as usize; //vx register
                        if emu_state.gpr[reg_index] != val {
                            emu_state.pc += 2;
                        }
                    }
                }
            }

            0x5000 => match opcode {
                _ => {
                    let reg_index = ((opcode & 0x0F00) >> 8) as usize; // vx register 
                    let reg_index_2 = ((opcode & 0x00F0) >> 4) as usize; // vy register 


                    if emu_state.gpr[reg_index] == emu_state.gpr[reg_index_2]{

                        emu_state.pc +=2;
                    }
                }
            },

            0x6000 => match opcode{
                _ => {
                    let val = (opcode & 0x00FF) as u8; //kk
                    let regdex = ((opcode & 0x0F00) >> 8) as usize; //grabs reg Vx
                    //let resint = emu_state.gpr[regdex] + val; //resulting int

                    emu_state.gpr[regdex] = val;

                }


            }

            0x7000 => match opcode{
                _ => {
                    let val = (opcode & 0x00FF) as u8; //kk
                    let regdex = ((opcode & 0x0F00) >> 8) as usize; //grabs reg Vx
                    let resint = emu_state.gpr[regdex] + val; //resulting int. Potential panic here

                    emu_state.gpr[regdex] = resint;

                }

            }

            0x8000 => match opcode & 0x000F{
                //let op = opcode & 0x000F;
                0x0 => {
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy

                    emu_state.gpr[regdex1] = emu_state.gpr[regdex2]; //stores Vy in Vx

                }

                0x1 => {
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy

                    emu_state.gpr[regdex1] = emu_state.gpr[regdex1] | emu_state.gpr[regdex2]; //bitwise OR,store in Vx

                }

                0x2 => {
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                    emu_state.gpr[regdex1] = emu_state.gpr[regdex1] & emu_state.gpr[regdex2];//Bitwise AND
                }

                0x3=> {
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                    emu_state.gpr[regdex1] = emu_state.gpr[regdex1] ^ emu_state.gpr[regdex2]; //bitwise XOR
                }

                0x4=>{
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                    let res = emu_state.gpr[regdex1] as u16 + emu_state.gpr[regdex2] as u16; //casting to u16 to avoid overflow
                    if res > 255{

                        emu_state.gpr[15] = 1;
                    }
                    else{
                        emu_state.gpr[15] = 0;
                    }

                    emu_state.gpr[regdex1] = (res & 0xFF) as u8;

                }

                0x5=>{

                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                    if emu_state.gpr[regdex1] > emu_state.gpr[regdex2] {
                        emu_state.gpr[15] =1;


                    }
                    else{

                        emu_state.gpr[15] =0;
                    }


                    let sub = emu_state.gpr[regdex1] - emu_state.gpr[regdex2];

                    emu_state.gpr[regdex1] = sub;
                }

                0x6 => {
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    //let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy

                    let lsb = emu_state.gpr[regdex1] & 0x01; //grab lsb


                    if lsb == 1{
                        emu_state.gpr[15] =1; //VF is 1

                    }
                    else{

                        emu_state.gpr[15] = 0;
                    }

                    emu_state.gpr[regdex1] = emu_state.gpr[regdex1] / 2; //div by 2
                }

                0x7 =>{

                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy

                    if emu_state.gpr[regdex2] > emu_state.gpr[regdex1]{
                        emu_state.gpr[15] = 1;

                    }
                    else{

                        emu_state.gpr[15] = 0;
                    }

                    emu_state.gpr[regdex1] = emu_state.gpr[regdex2] - emu_state.gpr[regdex1];

                }

                0xE =>{//TODO: unfinished opcode
                    let regdex1 =((opcode & 0x0F00) >> 8) as usize; //reg vx
                    //let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                    //grab MSB
                    let msb = (emu_state.gpr[regdex1] & 0x80) >> 7;  // Extract MSB and shift to position 0
                    if msb == 1 {  // Now this works!
                        emu_state.gpr[15] = 1;
                    }
                    else {
                        emu_state.gpr[15] = 0;
                    }

                    emu_state.gpr[regdex1] = emu_state.gpr[regdex1] *2;
                }

                _ => {



                }


            }

            0x9000 => match opcode{
                _ => {
                    let regdex1 = ((opcode & 0x0F00) >> 8) as usize;
                    let regdex2 = ((opcode & 0x00F0) >> 4) as usize;

                    if emu_state.gpr[regdex1] != emu_state.gpr[regdex2]{
                        emu_state.pc +=2;
                    }
                }

            }

            0xA000 => match opcode{
                _ => {
                    emu_state.ir = opcode & 0x0FFF;


                }
            }


            _ => {}
        }
    }
}
