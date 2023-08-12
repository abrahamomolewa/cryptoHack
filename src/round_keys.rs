

/* Complete the add_round_key function, then use the matrix2bytes function to get your next flag. */


fn matrix2string(matrix: &Vec<Vec<u8>>) -> String {
    matrix.iter()
        .flat_map(|row| row.iter())
        .map(|&val| val as char)
        .collect()
 }
 
 fn add_round_key(s: &Vec<Vec<u8>>, k: &Vec<Vec<u8>> ) -> Vec<Vec<u8>> {
  let mut result =  Vec::new();
   for i in 0..4 {
     let mut row = Vec::new();
 
     for j in 0..4 {
        row.push(s[i][j] ^ k[i][j]);
     }
     result.push(row)
 
   }
 result
 }
 
 
 
 
    fn main() {
       let s = vec![
           vec![206, 243, 61, 34],
           vec![171, 11, 93, 31],
           vec![16, 200, 91, 108],
           vec![150, 3, 194, 51],
       ];
       
       let k = vec![
           vec![173, 129, 68, 82],
           vec![223, 100, 38, 109],
           vec![32, 189, 53, 8],
           vec![253, 48, 187, 78],
       ];
       
       let result = add_round_key(&s, &k);
       let final_result = matrix2string(&result);
 
       println!("{:?}", final_result);
   }
   
 
