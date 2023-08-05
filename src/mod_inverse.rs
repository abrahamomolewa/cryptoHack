/*
What is the inverse element: 3 * d ≡ 1 mod 13?
 */

fn mod_inverse(base: u64, modulus: u64) -> Option<u64> {
    let mut d = 1;
    while d < modulus {
        if (base * d) % modulus == 1 {
            return Some(d);
        }
        d += 1;
    }
    None
}

pub fn main() {
    let base = 3;
    let modulus = 13;

    match mod_inverse(base, modulus) {
        Some(inverse) => println!("Inverse of {} in F{} is: {}", base, modulus, inverse),
        None => println!("Inverse of {} in F{} does not exist.", base, modulus),
    }
}
