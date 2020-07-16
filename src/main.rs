mod num;
mod util;

use std::env;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
fn main() {
   let v: Vec<String> = env::args().collect();
   let args: Vec<&str> = v.iter().map(AsRef::as_ref).collect();
 
   let (i, o, f, d, v) = util::parse_input(args);

   if f == None {
      println!("{}", util::convert(i, o, v, d));
   }
   else {
      let mut path = PathBuf::new();
      path.push(env::current_dir().unwrap().as_path());
      path.push(f.unwrap());

      for line in BufReader::new(File::open(path).unwrap()).lines() {
         line.unwrap().as_bytes().iter().for_each(|b| print!("{}",o.format_as(b)));
         println!();
      }
   } 
}

//-----------------------------------------------------------------------------------------------------------------//
//--------------------------Tests----------------------------------------------------------------------------------//
//-----------------------------------------------------------------------------------------------------------------//

#[cfg(test)]
mod full_send {
   use super::*;

   #[test]
   fn test1() {

   } 

}
