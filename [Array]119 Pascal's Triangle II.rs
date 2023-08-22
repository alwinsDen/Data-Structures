impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i64> {
        let mut ans = Vec::new();
        for vls in 1..=(row_index+1) as i64 {
            let mut cc=1;
            for vss in 1..=vls {
                if vls==(row_index+1) as i64 {
                    ans.push(cc);
                    // println!("{}",vls);
                    cc=(cc * (vls - vss)) / vss;
                }
            }
        }
        return ans;
    }
}
