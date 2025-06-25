use std::fs::File;
use std::io::{Read, Result};

pub fn dump_rom() -> Result<Vec<u8>> {
   let rom_path = std::env::current_dir()?.join("Lunar Lander (Udo Pernisz, 1979).ch8");
   println!("{:?}",rom_path);
   let mut state_vec: Vec<u8> = Vec::new();
   let mut game =  File::open(&rom_path)?;
   game.read_to_end(&mut state_vec)?;
   
   Ok(state_vec)
}