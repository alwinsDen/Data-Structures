// Topological sort on Directed Acyclic Graph
#[cfg(test)]
mod test{
    #[test]
    fn test1(){
        let gph = DAG::new(vec![(0, 2), (2, 3), (3, 4), (1, 2)]);
    }
    use std::collections::{HashMap};
    pub struct DAG{
        graph: Option<HashMap<u8,Vec<u8>>>
    }
    impl DAG{
        pub fn new(graph_info: Vec<(u8,u8)>)->Vec<u8>{
            let mut adj_map: HashMap<u8,Vec<u8>> = HashMap::new();
            for value in graph_info{
                let source_vertex = &mut adj_map.entry(value.0).or_insert(vec![]);
                source_vertex.push(value.1);
            }

            let graph = DAG{
                graph: Some(adj_map)
            };
            return graph.get_topological_order();
        }

        //get_topological_order method
        pub fn get_topological_order(&self) -> Vec<u8>{
            let src_nodes = self.graph.as_ref().unwrap().keys();
            let mut stack : Vec<u8> = vec![];
            for node in src_nodes {
                self.get_order(node,&mut stack);
            }
            stack.reverse();
            println!("The Stack!! {:?}",stack);
            return stack;
        }

        //get_method
        pub fn get_order(&self,node:&u8,stack:&mut Vec<u8>){
            let receiving_nodes = self.graph.as_ref().unwrap().get(node);
            if receiving_nodes!=None{
                for value in receiving_nodes.unwrap(){
                    self.get_order(value,stack);
                }
            }
            if !stack.contains(node){
                stack.push(*node);
            }
        }
    }
}
