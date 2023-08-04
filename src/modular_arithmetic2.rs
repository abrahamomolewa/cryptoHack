/*

Now take the prime p = 65537. Calculate 27324678765465536 mod 65537.

 */

fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
 
    let mut result = 1;
    let mut base = base % modulus;
 
    for i in 0..64 {
        if (exponent >> i) & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
    }
 
    result
 }
 
 pub fn main() {
    let base = 273246787654;
    let exponent = 65536;
    let modulus = 65537;
 
    let result = mod_pow(base, exponent, modulus);
 
    println!("273246787654^65536 mod 65537 = {}", result);
 }
 