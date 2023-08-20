//tests
#[cfg(test)]
mod test{
    //
    use std::collections::VecDeque;
    use std::collections::HashSet;
    // dfs algorithm
    #[derive(Copy,Clone,PartialEq,Eq,Hash,Debug)]
    pub struct Vertex(u32);

    #[derive(Copy,Clone,PartialEq,Eq,Hash)]
    pub struct Edge(u32,u32);

    #[derive(Clone)]
    pub struct Graph{
        vertices: Vec<Vertex>,
        edges: Vec<Edge>
    }
    impl Graph {
        pub fn new(vertices: Vec<Vertex>,edges: Vec<Edge>) -> Self {
            Graph {vertices, edges}
        }
    }
    // here From<u32> is for type conversion.
    impl From<u32> for Vertex {
        fn from(item: u32)->Self {
            Vertex(item)
        }
    }

    impl Vertex {
        pub fn value(&self)->u32{
            self.0
        }
        pub fn neighbors(&self,graph:&Graph)->VecDeque<Vertex>{
            graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into()) //here into is utilizing From<u32> of Vertex trait.
            .collect()
        }
    }

    impl From<(u32,u32)> for Edge {
        fn from(item: (u32,u32)) -> Self {
            Edge(item.0,item.1)
        }
    }

    pub fn df_search(graph: &Graph,root: Vertex, objective:Vertex)->Option<Vec<u32>>{
        let mut visited: HashSet<Vertex> = HashSet::new();
        let mut history: Vec<u32> = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        //here root is the starting point of the DFS df_search
        //here objective is the element that is being searched.
        while let Some(current_vertex) = queue.pop_front(){
            history.push(current_vertex.value());

            if current_vertex==objective {
                return Some(history);
            }

            //for each over the neighbors of current vertex
            for neighbor in current_vertex.neighbors(graph).into_iter().rev(){
                if visited.insert(neighbor) {
                    queue.push_front(neighbor);
                }
            }
        }
        None
    }

    #[test]
    fn find_1_fail(){
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;

        let correct_path = vec![1, 2, 4, 5, 3, 6, 7];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            df_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }
}
