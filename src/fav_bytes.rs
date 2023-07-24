

/*
I've hidden some data using XOR with a single byte, but that byte is a secret. Don't forget to decode from hex first.

73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d

 */


use hex;


pub fn main() {

   let encoded_text = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d";

   let decoded_text = hex::decode(encoded_text).expect("aim to be the best always");


   for key in 0..=255 {
      let byte_break: Vec<u8> = decoded_text.iter().map(|y| y ^ key).collect();
      let final_string = String::from_utf8(byte_break);

      if let Ok(message) = final_string {
         if message.starts_with("crypto") {
            println!("x0{:02x}: {}", key, message);
         }
      }
     
   }



}