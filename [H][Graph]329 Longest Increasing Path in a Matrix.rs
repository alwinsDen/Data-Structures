//https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
//TIME: Beats 23.08% of users with Rust
//MEM: Beats 30.77% of users with Rust

use std::collections::{HashMap};
use std::cmp;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut rows = matrix.len();
        let mut cols = matrix[0].len();
        let mut prevVls : HashMap<(usize, usize), i32> = HashMap::new();
        let mut mx_v : i32 = 0;

        pub fn lng_mx_ln(matrix: &Vec<Vec<i32>>, 
            rv: usize, 
            cv: usize, 
            preVlss : &mut HashMap<(usize, usize),i32>) 
            -> i32
            {
            let lfst : Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            if preVlss.contains_key(&(rv, cv)) {
                return *preVlss.get(&(rv,cv)).unwrap();
            }
            let mut msx = 0;
            for vls in lfst.iter(){
                let nsft : usize = (rv as i32 + vls.0) as usize;
                let msft : usize = (cv as i32 + vls.1) as usize;
                if (
                    nsft<matrix.len() 
                    && msft<matrix[0].len()
                    && nsft>=0
                    && msft>=0
                    && matrix[rv][cv] < matrix[nsft][msft] 
                ){
                    //longest of the 4 directions of 1 spot
                    msx = cmp::max(msx, lng_mx_ln(matrix, nsft, msft, preVlss));
                }
            }
            preVlss.insert((rv, cv), 1 + msx);
            return 1 + msx;
        }

        for rv in 0..rows{
            for cv in 0..cols{
                //longest of the 9 spots
                mx_v = cmp::max(mx_v, lng_mx_ln(&matrix, rv, cv, &mut prevVls));
            }
        }
        return mx_v;
    }
}
