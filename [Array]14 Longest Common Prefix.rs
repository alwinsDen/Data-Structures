impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = Vec::new();
        'looper: for (index,vls) in strs[0].chars().enumerate() {
            for vals in strs.iter(){
                if index<vals.len() && vls==vals.chars().nth(index).unwrap(){}
                else {
                    break 'looper;
                }
            }
            ans.push(vls);
        }
        return ans.iter().collect::<String>();
    }
}
