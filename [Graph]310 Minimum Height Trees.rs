// https://leetcode.com/problems/minimum-height-trees/
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n==1 {return vec![0]}
        let mut hsh : HashMap<i32,HashSet<i32>> = HashMap::new();
        for vls in edges.iter(){
            hsh.entry(vls[0]).or_insert(HashSet::new()).insert(vls[1]);
            hsh.entry(vls[1]).or_insert(HashSet::new()).insert(vls[0]);
        }
        let mut leaves : Vec<i32> = hsh.iter()
            .filter_map(|(k,v)| if v.len()==1{ Some(*k) }else {None})
            .collect();
        let mut m = n;
        while m > 2 {
            m -= leaves.len() as i32;
            let mut new_leaves : Vec<i32> = Vec::new();
            for vls in leaves.iter(){
                let &neighbor = hsh[vls].iter().next().unwrap();
                hsh.get_mut(&neighbor).unwrap().remove(vls);
                if hsh[&neighbor].len()==1{
                    new_leaves.push(neighbor);
                }
            }
            leaves = new_leaves;
        }
        return leaves;
    }
}
