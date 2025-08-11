use crate::emustatus::{self, Chip8Emu};
use rand::prelude::*;
use std::fs::File;
use std::io::{Read, Result};

pub fn dump_rom(entry_string: String) -> Result<Vec<u8>> {
    let rom_path = entry_string;
    println!("{:?}", rom_path);
    let mut state_vec: Vec<u8> = Vec::new();
    let mut game = File::open(&rom_path)?;
    game.read_to_end(&mut state_vec)?;
    Ok(state_vec)
}

pub fn execute_instructions(emu_state: &mut Chip8Emu) {
    let opcode = ((emu_state.memory[emu_state.pc as usize] as u16) << 8)
        | (emu_state.memory[emu_state.pc as usize + 1] as u16);

    emu_state.pc += 2;  // Default increment
    parser_gen(emu_state, opcode);
}

pub fn parser_gen(emu_state: &mut Chip8Emu, opcode: u16) {
    //combine into the full instruction

    match opcode >> 12 {
        //extract highest nibble
        0x0000 => {
            match opcode {
                0x00E0 => {
                    println!("Clear screen!");
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
                if emu_state.gpr[reg_index] == emu_state.gpr[reg_index_2] {
                    emu_state.pc += 2;
                }
            }
        },

        0x6000 => match opcode {
            _ => {
                println!("Loading");
                let val = (opcode & 0x00FF) as u8; //kk
                let regdex = ((opcode & 0x0F00) >> 8) as usize; //grabs reg Vx
                emu_state.gpr[regdex] = val;
            }
        },

        0x7000 => match opcode {
            _ => {
                let val = (opcode & 0x00FF) as u8; //kk
                let regdex = ((opcode & 0x0F00) >> 8) as usize; //grabs reg Vx
                let resint = emu_state.gpr[regdex] + val; //resulting int. Potential panic here
                emu_state.gpr[regdex] = resint;
            }
        },

        0x8000 => match opcode & 0x000F {
            0x0 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                emu_state.gpr[regdex1] = emu_state.gpr[regdex2]; //stores Vy in Vx
            }

            0x1 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                emu_state.gpr[regdex1] = emu_state.gpr[regdex1] | emu_state.gpr[regdex2];
                //bitwise OR,store in Vx
            }

            0x2 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                emu_state.gpr[regdex1] = emu_state.gpr[regdex1] & emu_state.gpr[regdex2];
                //Bitwise AND
            }

            0x3 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                emu_state.gpr[regdex1] = emu_state.gpr[regdex1] ^ emu_state.gpr[regdex2];
                //bitwise XOR
            }

            0x4 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                let res = emu_state.gpr[regdex1] as u16 + emu_state.gpr[regdex2] as u16; //casting to u16 to avoid overflow
                if res > 255 {
                    emu_state.gpr[15] = 1;
                } else {
                    emu_state.gpr[15] = 0;
                }
                emu_state.gpr[regdex1] = (res & 0xFF) as u8;
            }

            0x5 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                if emu_state.gpr[regdex1] > emu_state.gpr[regdex2] {
                    emu_state.gpr[15] = 1;
                } else {
                    emu_state.gpr[15] = 0;
                }
                let sub = emu_state.gpr[regdex1] - emu_state.gpr[regdex2];
                emu_state.gpr[regdex1] = sub;
            }

            0x6 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let lsb = emu_state.gpr[regdex1] & 0x01; //grab lsb
                if lsb == 1 {
                    emu_state.gpr[15] = 1; //VF is 1
                } else {
                    emu_state.gpr[15] = 0;
                }
                emu_state.gpr[regdex1] = emu_state.gpr[regdex1] / 2; //div by 2
            }

            0x7 => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; //reg vy
                if emu_state.gpr[regdex2] > emu_state.gpr[regdex1] {
                    emu_state.gpr[15] = 1;
                } else {
                    emu_state.gpr[15] = 0;
                }
                emu_state.gpr[regdex1] = emu_state.gpr[regdex2] - emu_state.gpr[regdex1];
            }

            0xE => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; //reg vx
                let msb = (emu_state.gpr[regdex1] & 0x80) >> 7; // Extract MSB and shift to position 0
                if msb == 1 {
                    emu_state.gpr[15] = 1;
                } else {
                    emu_state.gpr[15] = 0;
                }
                emu_state.gpr[regdex1] = emu_state.gpr[regdex1] * 2;
            }

            _ => {}
        },

        0x9000 => match opcode {
            _ => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize;
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize;
                if emu_state.gpr[regdex1] != emu_state.gpr[regdex2] {
                    emu_state.pc += 2;
                }
            }
        },

        0xA000 => match opcode {
            
            _ => {
                println!("General purpouse load");
                emu_state.ir = opcode & 0x0FFF;
            }
        },

        0xB000 => match opcode {
            _ => {
                emu_state.pc = (opcode & 0x0FFF) + (emu_state.gpr[0] as u16);
            }
        },

        0xC000 => match opcode {
            _ => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize;
                let mut rng = rand::rng();
                let mut num: u8;
                let dat = (opcode & 0x0FF) as u8;
                let mut nums: Vec<u8> = (0..255).collect();
                nums.shuffle(&mut rng);
                let res = nums.choose(&mut rng);
                match res {
                    Some(x) => num = *x, //gotta deref
                    None => panic!("Shouldn't be none here!"),
                }
                emu_state.gpr[regdex1] = dat & num;
            }
        },

        0xD000 => match opcode {
            _ => {
                let regdex1 = ((opcode & 0x0F00) >> 8) as usize; // Vx
                let regdex2 = ((opcode & 0x00F0) >> 4) as usize; // Vy
                let n_bytes = (opcode & 0x000F) as usize; // n

                let start_x = emu_state.gpr[regdex1] as usize;
                let start_y = emu_state.gpr[regdex2] as usize;
                let start_addr = emu_state.ir as usize;

                let mut collision = false;

                // For each sprite byte (row)
                for row in 0..n_bytes {
                    let sprite_byte = emu_state.memory[start_addr + row];

                    // For each bit in the byte (column)
                    for col in 0..8 {
                        // Extract the bit (MSB first)
                        let bit = (sprite_byte >> (7 - col)) & 1;

                        // Calculate screen position with wrapping
                        let screen_x = (start_x + col) % 64;
                        let screen_y = (start_y + row) % 32;

                        // Get current pixel value
                        let current_pixel = emu_state.display[screen_x][screen_y];

                        // XOR the bit with current pixel
                        let new_pixel = current_pixel ^ bit;

                        // Check for collision (pixel turned off)
                        if current_pixel == 1 && new_pixel == 0 {
                            collision = true;
                        }

                        // Update the display
                        emu_state.display[screen_x][screen_y] = new_pixel;
                    }
                }

                // Set VF based on collision
                emu_state.gpr[15] = if collision { 1 } else { 0 };
            }
        },

        0xE000 => match opcode & 0x00FF {
            0x9E => {
                // SKP Vx - Skip next instruction if key with value of Vx is pressed
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let key = emu_state.gpr[regdex];
                if key < 16 && emu_state.keypad[key as usize] {
                    emu_state.pc += 2;
                }
            }

            0xA1 => {
                // SKNP Vx - Skip next instruction if key with value of Vx is not pressed
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let key = emu_state.gpr[regdex];
                if key < 16 && !emu_state.keypad[key as usize] {
                    emu_state.pc += 2;
                }
            }

            _ => {}
        },

        0xF000 => match opcode & 0x00FF {
            0x07 => {
                // LD Vx, DT - Set Vx = delay timer value
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                emu_state.gpr[regdex] = emu_state.dt;
            }

            0x0A => {
                // LD Vx, K - Wait for a key press, store the value of the key in Vx
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let mut key_pressed = false;

                for i in 0..16 {
                    if emu_state.keypad[i] {
                        emu_state.gpr[regdex] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }

                if !key_pressed {
                    emu_state.pc -= 2; // Stay on this instruction until key is pressed
                }
            }

            0x15 => {
                // LD DT, Vx - Set delay timer = Vx
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                emu_state.dt = emu_state.gpr[regdex];
            }

            0x18 => {
                // LD ST, Vx - Set sound timer = Vx
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                emu_state.st = emu_state.gpr[regdex];
            }

            0x1E => {
                // ADD I, Vx - Set I = I + Vx
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                emu_state.ir = emu_state.ir.wrapping_add(emu_state.gpr[regdex] as u16);
            }

            0x29 => {
                // LD F, Vx - Set I = location of sprite for digit Vx
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let digit = emu_state.gpr[regdex] & 0x0F; // Only use lower 4 bits (0-F)
                emu_state.ir = 0x50 + (digit as u16 * 5); // Font data starts at 0x50, each char is 5 bytes
            }

            0x33 => {
                // LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let val = emu_state.gpr[regdex];
                let addr = emu_state.ir as usize;

                emu_state.memory[addr] = val / 100; // hundreds digit
                emu_state.memory[addr + 1] = (val / 10) % 10; // tens digit
                emu_state.memory[addr + 2] = val % 10; // ones digit
            }

            0x55 => {
                // LD [I], Vx - Store registers V0 through Vx in memory starting at location I
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let addr = emu_state.ir as usize;

                for i in 0..=regdex {
                    emu_state.memory[addr + i] = emu_state.gpr[i];
                }
            }

            0x65 => {
                // LD Vx, [I] - Read registers V0 through Vx from memory starting at location I
                let regdex = ((opcode & 0x0F00) >> 8) as usize;
                let addr = emu_state.ir as usize;

                for i in 0..=regdex {
                    emu_state.gpr[i] = emu_state.memory[addr + i];
                }
            }

            _ => {}
        },

        _ => {}
    }
}
