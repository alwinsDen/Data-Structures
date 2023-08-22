/* The knows API is defined for you.
       knows(a: i32, b: i32)->bool;
    to call it use self.knows(a,b)
*/

use std::collections::{HashMap};
impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let mut hsh : HashMap<i32,i32> = HashMap::new();
        let mut ahsh : HashMap<i32,i32> = HashMap::new();
        for vls in 0..n{
            for its in 0..n{
                if vls!=its{
                    if self.knows(vls,its){
                        hsh.entry(its).and_modify(|exs| *exs+=1).or_insert(1);
                        ahsh.entry(vls).and_modify(|exs| *exs+=1).or_insert(1);
                    }
                }
            }
        }
        let mut arr1 : Vec<_> = hsh.into_iter().collect();
        arr1.sort_by(|(_,a),(_,b)| b.cmp(a));
        if arr1.len()>0 && arr1[0].1==n-1 && !ahsh.contains_key(&arr1[0].0){
            if arr1.len()>1{
                if arr1[1].1==arr1[0].1 {
                    return -1;
                }
            }
            return arr1[0].0
        }
        return -1;
    }
}
