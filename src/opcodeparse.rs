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


pub fn parser_gen(x:Vec<u8>) {
   for z in 512..x.len(){
      let f_p = x[z] as u16;
      let s_p = x[z+1] as u16;

      let opcode: u16 = (f_p << 8) | (s_p);


      match opcode >> 12{
         0x0000 =>{

            match opcode{
               0x00E0 =>{ //Clear display



               }



            }



         }

      }

   }




}