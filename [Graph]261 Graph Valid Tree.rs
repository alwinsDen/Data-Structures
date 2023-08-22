use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut vct : HashMap<i32,Vec<i32>> = HashMap::new();
        for edge in edges.iter(){
            //undirected edges
            vct.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
            vct.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
        }
        //now we check whether all the values have been visited via DFS
        let mut vis_set : HashSet<i32>= HashSet::new();
        fn dfs(start: i32,vis_set: &mut HashSet<i32>,vct: &HashMap<i32,Vec<i32>>){
            if vis_set.contains(&start){
                return;
            }
            vis_set.insert(start);
            if let Some(vss)=vct.get(&start){
                for vls in vss.iter(){
                    dfs(*vls,vis_set,vct);
                }
            }
        }
        dfs(0,&mut vis_set,&vct);
        //false if a node is not attached
        if n!=vis_set.len() as i32{
            return false;
        }
        //now detect cycles
        let mut vvsit_list:HashSet<i32>=HashSet::new();
        let mut parent_arr:HashMap<i32,i32>=HashMap::new();
        fn cycle_detect(node: i32,visit_list: &mut HashSet<i32>,vct: &HashMap<i32,Vec<i32>>,parent_arr:&mut HashMap<i32,i32>)->bool{
            visit_list.insert(node);
            if let Some(adj_eles) = vct.get(&node){
                for adj in adj_eles.iter(){
                    if !visit_list.contains(&adj){
                        parent_arr.insert(*adj,node);
                        if cycle_detect(*adj,visit_list,vct,parent_arr){
                            return true;
                        }
                        //after the parents and the child node is mapped.
                    } else if let Some(parent) = parent_arr.get(&node){
                        if parent!=adj {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
        for vls in vct.keys(){
            if !vvsit_list.contains(&vls){
                let cycles = cycle_detect(*vls,&mut vvsit_list,&vct,&mut parent_arr);
                if cycles{
                    return false;
                }
            }
        }
        return true;
    }
}
