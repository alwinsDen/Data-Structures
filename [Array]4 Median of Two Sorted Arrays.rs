use std::collections::{HashSet};
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut l_set : Vec<i32> = Vec::new();
        for &vls in nums1.iter() {
            l_set.push(vls);
        }
        for &vls in nums2.iter() {
            l_set.push(vls);
        }
        // let mut l_set : Vec<i32> = hss.into_iter().collect();
        l_set.sort();
        let mp = l_set.len();
        if mp%2==0 {
           return ( 
               l_set[(mp/2) - 1] as f64 + l_set[(mp/2)] as f64
            )/2.0
        } else {
            return l_set[(mp/2)] as f64
        }
    }
}
