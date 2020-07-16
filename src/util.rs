pub use crate::num::*;

pub fn convert(i: Num, o: Num, values: &str, delim: &str)->String {   
  
   let mut result = String::new();
   to_dec(i, values, delim)
      .iter()
      .for_each(|num| {
         result += o.format_as(num).as_ref();
          
         if o != Num::Ascii {result += " "}
		}
	);
 
   if o != Num::Ascii {result.pop();} //Remove the final " " that gets appended
 
   return result
}

  ///Parses the user input
 ///Output:
 /// *i: in-the numbering system of the input values
 /// *o: out-the numbering system to convert the input values to
 /// *f: file-the file to read input values from (values can be provided directly to command)
 /// *d: delimeter-How the data is split up
pub fn parse_input(args: Vec<&str>)->(Num, Num, Option<&str>, &str, &str) {
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
      else if args[j] == "-d" || args[j] == "-delim" || args[j] == "delimiter"{
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

use std::{num::ParseIntError};
fn to_dec(i: Num, v: &str, d: &str) -> Vec<u8> {
 
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

//-----------------------------------------------------------------------------------------------------------------//
//--------------------------Tests----------------------------------------------------------------------------------//
//-----------------------------------------------------------------------------------------------------------------//
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
mod to_dec {
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
mod convert {
   use super::*;

   #[test]
   fn test1() {

      let i = Num::Ascii;
      let o = Num::Hex;
      let d = " ";
      let v = "Hello There";
   
      assert_eq!(convert(i, o, v, d), String::from("48 65 6c 6c 6f 20 54 68 65 72 65"));
   }

   #[test]
   fn test2() {

      let i = Num::Hex;
      let o = Num::Ascii;
      let d = " ";
      let v = "48 65 6c 6c 6f 20 54 68 65 72 65";

      assert_eq!(convert(i, o, v, d), String::from("Hello There"));
	}
	
	#[test]
   fn test3() {

      let i = Num::Hex;
      let o = Num::Ascii;
      let d = "::";
      let v = "48::65::6c::6c::6f::20::54::68::65::72::65";

      assert_eq!(convert(i, o, v, d), String::from("Hello There"));
   }
}


 
