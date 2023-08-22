impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut s_inx = i32::MIN;
        let mut ins=nums.len()-1;
        'jester: while ins > 0 {
            if nums[ins-1]<nums[ins] {
                s_inx = ins as i32;
                break 'jester;
            }
            ins-=1;
        }
        println!("{}",s_inx);
        if s_inx==i32::MIN {
            nums.sort();
        } else {
            let c_val=nums[s_inx as usize-1];
            let c_nums = nums.clone();
            'mist: for usz in s_inx as usize+1..c_nums.len(){
                println!("{}",usz);
                if c_nums[usz]>c_val {
                    let mut temp = nums[s_inx as usize];
                    nums[s_inx as usize] = nums[usz];
                    nums[usz] = temp;
                    // break 'mist;
                }
            }
            println!("{:?}",nums);
            nums.swap(s_inx as usize,s_inx as usize-1);
            nums[s_inx as usize..].sort();
        }
    }
}
