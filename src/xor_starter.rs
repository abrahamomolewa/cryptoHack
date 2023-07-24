

/* Question 1. Given the string "label", XOR each character with the integer 13.
 Convert these integers back to a string and submit the flag as crypto{new_string} */

pub fn main() { 
 let c = ['l', 'a', 'b', 'e', 'l']; 
 let n: u8 = 13; 
 
 
 let resultss: String = c.iter().map( |&ch|  
 { 
     let cc = ch as u8;
     let mut result = [0; 8];
 
     for i in 0..8 {
         let bit = ((cc >> i) & 1) ^ ((n >> i) & 1);
         result[7 - i] = bit;
     }
 
     let results: String = result.iter().map(|c| c.to_string()).collect();
     let decimal_value = u8::from_str_radix(&results, 2).expect("");
     let character = char::from(decimal_value);
  
 character
   
 }).collect();
 
 println!("crypto{{{}}}", resultss);
 
}