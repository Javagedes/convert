#[derive(Clone, Copy, Debug, PartialEq)]
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
           _ => panic!("unknown type of numbering system")
       }
   }
 
   fn format_as(&self, n: &u8)->String {
       match self {
           &Num::Bin => {return format!("{:08b}", n)},
           &Num::Oct => {return format!("{:03o}", n)},
           &Num::Dec => {return format!("{}", n)},
           &Num::Hex => {return format!("{:02x}", n)},
           &Num::Ascii => {return format!("{}", *n as char)}
       }
   }
}
 
use std::{num::ParseIntError};
pub fn to_dec(i: Num, v: &str, d: &str) -> Vec<u8> {
 
   match i {
       Num::Ascii => {
           return v
               .as_bytes()
               .to_vec();
       },
       _ => {
           return v.split(d)
               .map(|j| u8::from_str_radix(&j, i as u32))
               .collect::<Result<Vec<u8>, ParseIntError>>()
               .unwrap();
       }
   }
}
 
fn convert(i: Num, o: Num, values: &str, delim: &str)->String {   
  
   let mut result = String::new();
   to_dec(i, values, delim)
       .iter()
       .for_each(|num| {
         result += o.format_as(num).as_ref();
         
         if o != Num::Ascii {result += " "}
      });

      if o != Num::Ascii {result.pop();} //Remove the final " " that gets appended

   return result
}

///Parses the user input
///Output:
/// *i: in-the numbering system of the input values
/// *o: out-the numbering system to convert the input values to
/// *f: file-the file to read input values from (values can be provided directly to command)
/// *d: delimeter-How the data is split up
fn parse_input(args: Vec<&str>)->(Num, Num, Option<&str>, &str, &str) {
   let mut i: Num = Num::Bin;
   let mut o: Num = Num::Ascii;
   let mut f: Option<&str> = None;
   let mut d: &str = " ";
   let mut v = "";
 
   let mut j = 1;
   while j < args.len() {  
      if args[j] == "-i" || args[j] == "-in" {
         j+=1;
         i = Num::from(args[j]);
      }
      else if args[j] == "-o" || args[j] == "-out"{
         j+=1;
         o = Num::from(args[j]);
      }
      else if args[j] == "-f" || args[j] == "-file"{
         j+=1;
         f = Some(args[j]);
      }
      else if args[j] == "-d" || args[j] == "-delim" || args[j] == "delimeter"{
         j+=1;
         d = args[j];
      }
      else {
         v = args[j]
      }
      j+=1;
   }

   return (i, o, f, d, v);
}
 
use std::env;
fn main() {
   let v: Vec<String> = env::args().collect();
   let args: Vec<&str> = v.iter().map(AsRef::as_ref).collect();
 
   let (i, o, f, d, v) = parse_input(args);

   use std::path::PathBuf;
   use std::io::BufReader;
   use std::io::BufRead;
   use std::fs::File;
   if f == None {
      println!("{}",convert(i, o, v, d));
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

#[cfg(test)]
mod parser {
   use super::*;

   #[test]
   fn test1() {
      let input: Vec<&str> = vec!["exe loc","-i", "ascii", "-o", "hex", "Hello There"];

      let (i, o, f, d, v) = parse_input(input);

      assert_eq!(i, Num::Ascii);
      assert_eq!(o, Num::Hex);
      assert_eq!(f, None);
      assert_eq!(d, " ");
      assert_eq!(v, "Hello There");
   }

   #[test]
   fn test2() {
      let input: Vec<&str> = vec!["exe loc","-i", "ascii", "-o", "hex", "-f", "file.txt"];

      let (i, o, f, d, v) = parse_input(input);

      assert_eq!(i, Num::Ascii);
      assert_eq!(o, Num::Hex);
      assert_eq!(f, Some("file.txt"));
      assert_eq!(d, " ");
      assert_eq!(v, "");
   }

   #[test]
   fn test3() {
      let input: Vec<&str> = vec!["exe loc","-i", "ascii", "-o", "hex", "Hello There", "-d", "."];

      let (i, o, f, d, v) = parse_input(input);

      assert_eq!(i, Num::Ascii);
      assert_eq!(o, Num::Hex);
      assert_eq!(f, None);
      assert_eq!(d, ".");
      assert_eq!(v, "Hello There");
   }
}

#[cfg(test)]
mod convert_to_dec {
   use super::*;
   
   #[test]
   fn test_ascii() {
      let i = Num::Ascii;
      let v = "Hello There";
      let d = " "; // Not important for ascii

      let dec = to_dec(i, v, d);

      assert_eq!(dec, vec![72, 101, 108, 108, 111, 32, 84, 104, 101, 114, 101]);
   }

   #[test]
   fn test_bin() {
      let i = Num::Bin;
      let v =  "01001000 01100101 01101100 01101100 01101111";
      let d = " ";

      let dec = to_dec(i, v, d);

      assert_eq!(dec, vec![72, 101, 108, 108, 111]);
   }

   #[test]
   fn test_oct() {
      let i = Num::Oct;
      let v =  "110 145 154 154 157";
      let d = " ";

      let dec = to_dec(i, v, d);

      assert_eq!(dec, vec![72, 101, 108, 108, 111]);
   }

   #[test]
   fn test_hex() {
      let i = Num::Hex;
      let v =  "48 65 6C 6C 6F";
      let d = " ";

      let dec = to_dec(i, v, d);

      assert_eq!(dec, vec![72, 101, 108, 108, 111]);
   }

   #[test]
   fn test_delim1() {
      let i = Num::Hex;
      let v =  "48.65.6C.6C.6F";
      let d = ".";

      let dec = to_dec(i, v, d);

      assert_eq!(dec, vec![72, 101, 108, 108, 111]);
   }

   #[test]
   fn test_delim2() {
      let i = Num::Hex;
      let v =  "48::65::6C::6C::6F";
      let d = "::";

      let dec = to_dec(i, v, d);

      assert_eq!(dec, vec![72, 101, 108, 108, 111]);
   }
}

#[cfg(test)]
mod full_send {
   use super::*;

   #[test]
   fn test1() {

      let args: Vec<&str> = vec!["exe loc","-i", "ascii", "-o", "hex", "Hello There"];
    
      let (i, o, f, d, v) = parse_input(args);
   
      if f == None {
         assert_eq!(convert(i, o, v, d), String::from("48 65 6c 6c 6f 20 54 68 65 72 65"));
      }
      else {
         panic!("The File variable should not be changed here");
      }
   }

   #[test]
   fn test2() {

      let args: Vec<&str> = vec!["exe loc","-i", "hex", "-o", "ascii", "48 65 6c 6c 6f 20 54 68 65 72 65"];
    
      let (i, o, f, d, v) = parse_input(args);

      if f == None {
         assert_eq!(convert(i, o, v, d), String::from("Hello There"));
      }
      else {
         panic!("The File variable should not be changed here");
      }
   }
}
