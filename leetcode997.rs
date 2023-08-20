use std::collections::HashMap;
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len()>0{
            let mut hsh : HashMap<i32,i32>  = HashMap::new();
            let mut uniHsh : HashMap<i32,i32>  = HashMap::new();
            for (index,vls) in trust.iter().enumerate(){
                match hsh.get_mut(&vls[1]) {
                    Some(ex_value)=> {
                        *ex_value += 1;
                    },
                    None => {hsh.entry(vls[1]).or_insert(1);}
                }
                uniHsh.insert(vls[0],0);
            }
            let mut arr1 : Vec<_>= hsh.into_iter().collect();
            arr1.sort_by(|(_,val1), (_,val2)| val2.cmp(val1));
            if uniHsh.contains_key(&arr1[0].0)==false && arr1[0].1 == n-1 {
                return arr1[0].0
            }
            else {
                return -1;
            }
        } else if n==1 {
            return 1;
        } else {
            return -1;

        }
    }
}
