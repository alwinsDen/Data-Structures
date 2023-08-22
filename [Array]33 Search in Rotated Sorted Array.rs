impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        fn binary_search<T:Ord>(target:T, arr:Vec<T>, left: &usize, right:&usize)
                                ->Option<usize>
        {
            if left>=right
            {
                return None
            }
            let mid = left + (right-left)/2;
            println!("{} {} {}",mid,left,right);
            match target.cmp(&arr[mid]) {
                Ordering::Less=>binary_search(target,arr,left,&mid),
                Ordering::Greater=>binary_search(target,arr,&(mid+1),right),
                Ordering::Equal=>return Some(mid)
            }
        }
        let mut sp_inx = i32::MIN;
        let len = nums.len();
        for (inx,vls) in nums.iter().enumerate()
        {
            if inx!=nums.len()-1 && *vls > nums[inx+1]{
                println!("This reaches here");
                sp_inx = inx as i32;
            }
        }

        if sp_inx==i32::MIN {
            if binary_search(target,nums.clone(),&0,&len)==None{
                return -1i32;
            }
            return binary_search(target,nums.clone(),&0,&len).unwrap() as i32;
        }

        if let Some(vls)=binary_search(target,nums.clone(),&0,&(sp_inx as usize+1)){
            return vls as i32;
        }else{
            if binary_search(target,nums.clone(),&(sp_inx as usize+1),&len)==None{
                return -1i32;
            } else {
                return binary_search(target,nums.clone(),&(sp_inx as usize+1),&len).unwrap() as i32;
            }
        }
    }
}
