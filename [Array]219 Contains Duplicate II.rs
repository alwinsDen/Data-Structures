use std::collections::{HashMap};
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hs_m=HashMap::new();
        for (inx,vls) in nums.into_iter().enumerate(){
            match hs_m.get_mut(&vls){
                Some(vl)=> {
                    if (inx - *vl) as i32<=k {return true;}
                    *vl=inx;
                },
                None=> {
                    hs_m.insert(vls,inx);
                }
            }
        }
        false
    }
}
