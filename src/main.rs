#[derive(Clone, Copy)]
pub enum Num {
   Ascii,
   Bin = 2,
   Oct = 8,
   Dec = 10,
   Hex = 16
}
 
use std::ops::Deref;
impl Num {
   fn from(s: &str)->Num {
       match s.to_ascii_lowercase().deref(){
           "bin" => return Num::Bin,
           "binary" => return Num::Bin,
           "oct" => return Num::Oct,
           "octal" => return Num::Oct,
           "dec" => return Num::Dec,
           "decimal" => return Num::Dec,
           "hex" => return Num::Hex,
           "hexadecimal" => return Num::Hex,
           "ascii" => return Num::Ascii,
           _ => panic!("unknown type of num")
       }
   }
 
   fn print_as(&self, n: &u8) {
       match self {
           &Num::Bin => {print!("{:08b} ", n)},
           &Num::Oct => {print!("{:03o} ", n)},
           &Num::Dec => {print!("{} ", n)},
           &Num::Hex => {print!("{:02x} ", n)},
           &Num::Ascii => {print!("{}", *n as char)}
       }
   }
}
 
use std::{num::ParseIntError};
pub fn to_dec(t: Num, s: &str) -> Vec<u8> {
 
   match t {
       Num::Ascii => {
           return s
               .as_bytes()
               .to_vec();
       },
       _ => {
           return s.split_ascii_whitespace()
               .map(|i| u8::from_str_radix(&i, t as u32))
               .collect::<Result<Vec<u8>, ParseIntError>>()
               .unwrap();
       }
   }
}
 
fn convert(convert_from: Num, convert_to: Num, values: &str) {   
  
   to_dec(convert_from, values)
       .iter()
       .for_each(|num| convert_to.print_as(num));
   println!()
}
 
use std::env;
fn main() {
   let v: Vec<String> = env::args().collect();
   let args: Vec<&str> = v.iter().map(AsRef::as_ref).collect();
 
   let mut j = 1;
   let mut i = Num::Bin;
   let mut o = Num::Ascii;
   let mut nums = "";
 
   while j < args.len() {  
       if args[j] == "-i" || args[j] == "-in" {
           j+=1;
           i = Num::from(args[j]);
       }
       else if args[j] == "-o" || args[j] == "-out"{
           j+=1;
           o = Num::from(args[j]);
       }
       else {
           nums = args[j]
       }
       j+=1;
   }
   convert(i, o, nums)
}
