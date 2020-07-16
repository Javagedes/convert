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
   pub fn from(s: &str)->Num {
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
 
   pub fn format_as(&self, n: &u8)->String {
       match self {
           &Num::Bin => {return format!("{:08b}", n)},
           &Num::Oct => {return format!("{:03o}", n)},
           &Num::Dec => {return format!("{}", n)},
           &Num::Hex => {return format!("{:02x}", n)},
           &Num::Ascii => {return format!("{}", *n as char)}
       }
   }
}

#[cfg(test)]
mod format {
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(Num::Bin.format_as(&255), "11111111");
        assert_eq!(Num::Oct.format_as(&255), "377");
        assert_eq!(Num::Dec.format_as(&255), "255");
        assert_eq!(Num::Hex.format_as(&255), "ff");
        assert_eq!(Num::Ascii.format_as(&64), "@");
    }
}