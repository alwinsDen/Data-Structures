//https://leetcode.com/problems/evaluate-division
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn dfs(start: &String, end: &String, status: &mut Vec<bool>, hsh: &HashMap<String, Vec<(String, f64)>>, visited: &mut HashSet<String>, ans: &mut Vec<f64>, main: &String, vcc: &mut Vec<f64>) -> bool {
        if visited.insert(start.to_string()) && status[1]==false {
            if let Some(t) = hsh.get(start){
                for tt in t.iter(){
                    if main==&tt.0 {
                        vcc.push(1.0/tt.1);
                    }
                    if end==&tt.0 && status[1] == false{
                        if vcc.len() > 0 {
                            ans.push(tt.1 * vcc[0]);
                            return true;
                        } else {
                            status[1] = true;
                            ans.push(tt.1);
                            return true;
                        }
                    }
                    let test = Self::dfs(&tt.0, end, status, hsh, visited, ans, main, vcc);
                    if test==true{
                        return test;
                    }
                }
            }
        }
        return false;
    }
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut hsh : HashMap<String,Vec<(String, f64)>>= HashMap::new();
        let mut hss : HashSet<String> = HashSet::new();
        for (inx, vls) in equations.iter().enumerate(){
            hsh.entry(vls[0].clone()).or_insert(Vec::new()).push((vls[1].clone(),values[inx]));
            hsh.entry(vls[1].clone()).or_insert(Vec::new()).push((vls[0].clone(),1.0/values[inx]));
            hss.insert(vls[0].clone()); hss.insert(vls[1].clone());
        }
        let mut ans : Vec<f64> = Vec::new();
        for vls in queries {
            let mut visited : HashSet<String> = HashSet::new();
            let mut rec = vec![false, false];
            let mut vcc : Vec<f64>= Vec::new();
            if(hss.contains(&vls[0]) && hss.contains(&vls[1])){
                let stp = Self::dfs(&vls[0], &vls[1], &mut rec, &hsh, &mut visited, &mut ans, &vls[0], &mut vcc);
            } else {
                ans.push(-1f64);
            }
        }
        return ans;
    }
}
