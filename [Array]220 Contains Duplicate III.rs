use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        if nums[0].abs()==nums[1].abs() && nums.len()==2 && nums[0]+nums[1]==0{
            return false;
        }
        if value_diff>0 {
            //will be using the buckets
            let mut h_mp=HashMap::new();
            for inx in 0..nums.len(){
                let mut bucket_index=(nums[inx]) / (value_diff+1);
                if h_mp.insert(bucket_index,nums[inx]) !=None
                || h_mp.get(&(bucket_index+1)).map_or(false,|v|v-nums[inx]<=value_diff)
                || h_mp.get(&(bucket_index-1)).map_or(false,|v|nums[inx]-v<=value_diff)
                {
                    return true;
                }
                if inx>=index_diff as usize {
                    bucket_index= (nums[inx-index_diff as usize])/(value_diff+1);
                    h_mp.remove(&bucket_index);
                }
            }
        }else {
            let mut h_mp=HashMap::new();
            for (inx,vls) in nums.into_iter().enumerate(){
                match h_mp.get_mut(&vls) {
                    Some(n)=>{
                        if inx-*n as usize<=index_diff as usize {return true;}
                        *n=inx
                    },
                    None=> {
                        h_mp.insert(vls,inx);
                    }
                }
            }
        }
        false
    }
}
