//https://leetcode.com/problems/reconstruct-itinerary/
//Beats 93.10% of users with Rust
//Beats 80.56% of users with Rust
use std::collections::HashMap;
impl Solution {
    fn dfs(anss: &mut Vec<String>, hsh: &mut HashMap<String, Vec<String>>, searchPoint: &String){
        while let Some(vls) = hsh.get_mut(searchPoint).and_then(|dest| dest.pop()) {
            Self::dfs(anss, hsh, &vls);
        } 
        anss.push(searchPoint.to_string());
    }
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut c_t = tickets;
        c_t.sort_by(|a,b| b.cmp(a));
        let mut hsh : HashMap<String, Vec<String>> = HashMap::new();
        for vls in c_t.iter() {
            hsh.entry(vls[0].clone()).or_insert(Vec::new()).push(vls[1].clone());
        }
        let mut ans : Vec<String> = Vec::new();
        Self::dfs(&mut ans, &mut hsh, &String::from("JFK"));
        ans.reverse();
        return ans;
    }
}
