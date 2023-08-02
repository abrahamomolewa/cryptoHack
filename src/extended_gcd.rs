fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
       let temp = b;
      b = a % b;
      a = temp;
    }
    a
 }
 
 pub fn main() {
 
    let x = 66528;
    let y = 52920;
 
    let result = gcd(x, y);
 
    println!("{}", result);
 
 }