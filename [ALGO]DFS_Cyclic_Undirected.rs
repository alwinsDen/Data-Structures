//dfs undirectional && cyclic
#[cfg(test)]
mod test {
    use std::collections::HashMap;
    type Vertex = u32;
    struct Graph {
        adj_list: HashMap<Vertex, Vec<Vertex>>
    }
    impl Graph{
        fn new() -> Self{
            Graph {
                adj_list: HashMap::new()
            }
        }
        fn add_edge(&mut self,from: Vertex,to: Vertex) {
            //since it is undirectional we add both the ends
            self.adj_list
            .entry(from)
            .or_insert(Vec::new())
            .push(to);

            self.adj_list
            .entry(to)
            .or_insert(Vec::new())
            .push(from);
        }
        fn dfs(&self,start:Vertex,visited:&mut HashMap<Vertex,bool>){
            if visited.contains_key(&start) {
                return;
            }
            visited.insert(start,true);
            println!("visited node: {}", start);
            if let Some(curr_val) = self.adj_list.get(&start) {
                for (index, &vls) in curr_val.iter().enumerate(){
                    self.dfs(vls,visited);
                }
            }
        }
        fn depth_first_search(&self,start: Vertex){
            let mut visited = HashMap::new();
            self.dfs(start,&mut visited);
        }
        fn has_cycle(&self)->bool{
            let mut visited = HashMap::new();
            let mut parent = HashMap::new();
            for &node in self.adj_list.keys(){
                if !visited.contains_key(&node){
                    if self.detect_cycle(node,&mut visited,&mut parent){
                        return true;
                    }
                }
            }
            return false;
        }
        //this algorithm is used to detect cycles
        fn detect_cycle(&self,node:Vertex,visited: &mut HashMap<Vertex,bool>,parent: &mut HashMap<Vertex,Vertex>)->bool{
            visited.insert(node,true);
            if let Some(neighbors) = self.adj_list.get(&node){
                for &neighbor in neighbors.iter(){
                    if !visited.contains_key(&neighbor){
                        parent.insert(neighbor,node);
                        if self.detect_cycle(neighbor,visited,parent){
                            return true;
                        }
                    } else if let Some(&parent_node) = parent.get(&node){
                        if parent_node!=neighbor{
                            return true;
                        }
                    }
                }
            }
            false
        }
    }

    //tests
    #[test]
    fn test_1(){
        let mut grph = Graph::new();
        grph.add_edge(1,2);
        grph.add_edge(2,3);
        grph.add_edge(3,4);
        grph.add_edge(4,2);
        grph.depth_first_search(3);
        println!("{}",grph.has_cycle());
    }
}
