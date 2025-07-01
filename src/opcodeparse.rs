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
        let f_p = emu_state.memory[z] as u16;
        let s_p = emu_state.memory[z + 1] as u16;

        let opcode: u16 = (f_p << 8) | (s_p);

        match opcode >> 12 {
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
                        emu_state.sp += 1; //stack pointer incremented
                        emu_state.stack[emu_state.sp as usize] = emu_state.pc; //current pc top of stack

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
                }
            },

            _ => {}
        }
    }
}
