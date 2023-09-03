use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut hsh : HashMap<i32,HashSet<i32>> = HashMap::new();
        for vls in edges.iter(){
            hsh.entry(vls[0]).or_insert(HashSet::new()).insert(vls[1]);
            hsh.entry(vls[1]).or_insert(HashSet::new()).insert(vls[0]);
        }
        fn dfs(stp: i32, v_set: &mut HashSet<i32>, hsh: &HashMap<i32,HashSet<i32>>) {
            if v_set.contains(&stp) {
                return;
            }
            v_set.insert(stp);
            if let Some(vlss) = hsh.get(&stp) {
                for value in vlss.iter(){
                    dfs(*value, v_set, hsh);
                }
            }
        }
        let mut ans = 0i32;
        let mut v_set : HashSet<i32> = HashSet::new();
        for (k,v) in hsh.iter(){
            if !v_set.contains(&k){
                dfs(*k, &mut v_set, &hsh);
                ans+=1;
            }
        }
        return ans;
    }
}