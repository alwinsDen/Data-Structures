use std::collections::HashMap;
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut hsh = HashMap::new();
        for (index,vls) in edges.iter().enumerate(){
            match hsh.get_mut(&vls[0]) {
                Some(ex_val)=> {
                    *ex_val+=1;
                },
                None=>{
                    hsh.entry(&vls[0]).or_insert(1);
                }
            }
            match hsh.get_mut(&vls[1]) {
                Some(ex_val)=> {
                    *ex_val+=1;
                },
                None=> {
                    hsh.entry(&vls[1]).or_insert(1);
                }
            }
        }
        let mut srtt : Vec<_> = hsh.into_iter().collect();
        srtt.sort_by(|&(_,vl1),&(_,vl2)| vl2.cmp(&vl1));
        return *srtt[0].0;
    }
}
