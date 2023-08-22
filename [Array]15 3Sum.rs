use std::collections::{HashMap};
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nms = nums;
        nms.sort_unstable_by(|a, b| a.cmp(b));
        let mut tst = HashMap::new();
        for (inx, vls) in nms.iter().enumerate() {
            let mut l = inx + 1;
            let mut r = nms.len() - 1;
            let m_val = nms[inx];
            // this here reduces the code's time to re-run loops for values
            if inx>0 && nms[inx] == nms[inx - 1] {
                continue;
            }
            while l < r {
                let summ = nms[l] + nms[r] + m_val;
                if summ == 0 {
                    let mut p_vec = Vec::from([nms[l], nms[r], m_val]);
                    p_vec.sort();
                    l+=1;
                    tst.insert(p_vec, inx);
                } else if summ > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        let mut ans=Vec::new();
        // here keys loops through the HashMap.
        for vls in tst.keys(){
            ans.push(vls.to_vec());
        }
        return ans;
    }
}
