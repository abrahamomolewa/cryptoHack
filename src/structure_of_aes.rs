/*
Write a matrix2bytes function to turn that matrix back into bytes, and submit the resulting plaintext as the flag.
 */

fn matrix2bytes(matrix: &Vec<Vec<u8>>) -> String {
    matrix.iter()
        .flat_map(|row| row.iter())
        .map(|&val| val as char)
        .collect()
 }
 
 fn main() {
    let matrix = vec![
        vec![99, 114, 121, 112],
        vec![116, 111, 123, 105],
        vec![110, 109, 97, 116],
        vec![114, 105, 120, 125],
    ];
 
    println!("{}", matrix2bytes(&matrix));
 }
 