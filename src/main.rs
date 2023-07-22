
use hex;

fn hex_xor_strings(a: &str, b: &str, ) -> Vec<u8> {

    let a_bytes = hex::decode(a).expect("invalid hex string");
    let b_bytes = hex::decode(b).expect("invalid hex String");
    
    assert_eq!(a_bytes.len(), b_bytes.len(), "Hexadecimal strings must have the same length for XOR");
    
    let xo_result: Vec<u8> = a_bytes.iter().zip(b_bytes.iter()).map(|(x,y)| x ^ y).collect();
    
     
       xo_result
    
    
    }
    
    fn hex_to_string(hex: &str) -> String {
       hex.chars().collect::<Vec<char>>().chunks(2).map(|chunk| { 
       let combined: String = chunk.into_iter().collect(); 
       u8::from_str_radix(&combined, 16).unwrap() as char
     
    }).collect()
    
    }
    

fn main() {

/* Question 1. Given the string "label", XOR each character with the integer 13.
 Convert these integers back to a string and submit the flag as crypto{new_string} */

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


 /* Question 2 
    
    Below is a series of outputs where three random keys have been XOR'd together and with the flag. Use the above properties to undo the encryption in the final line to obtain the flag.

KEY1 = a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313
KEY2 ^ KEY1 = 37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e
KEY2 ^ KEY3 = c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1
FLAG ^ KEY1 ^ KEY3 ^ KEY2 = 04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf

   Before you XOR these objects, be sure to decode from hex to bytes.
    
     */
     
     // to get key 2,  KEY2 = (KEY2 ^ KEY1) ^ KEY1 = key2

     // let key 1 = a and b = (KEY2 ^ KEY1);
     let a = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
     let b = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";
 
     let xor = hex_xor_strings(a, b);
     let final_result = hex::encode(xor);
 
     // To get key 3 (key2 ^ key3)^key2 = key3
     // let c = key2, and d = KEY2 ^ KEY3;
     let c = &final_result;
     let d = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
     let xor2 = hex_xor_strings(c, d);
     let final_result2 = hex::encode(xor2);
 
     // To get key1 ^ key2 ^ key3
     let e = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";
     let f = &final_result2;
     let xor3 = hex_xor_strings(e, f);
     let final_result3 = hex::encode(xor3);
 
     // To get the flag, (FLAG ^ KEY1 ^ KEY3 ^ KEY2) ^ (KEY1 ^ KEY2 ^ KEY3)
     let g = "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";
     let h = &final_result3;
     let xor4 = hex_xor_strings(g, h);
     let final_result4 = hex::encode(xor4);
 
     let solution = hex_to_string(&final_result4);
     println!("{}", solution);






}


