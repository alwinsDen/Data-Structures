/**
 * @param {number[][]} edges
 * @return {number[]}
 */
var findRedundantConnection = function(edges) {
    
    let mps = {}

    //here is the DfS function
    const dfs = (src, target, visited) => {
        if (src==target) return true;
        visited.add(src);

        for(let neigh of mps[src]){

            if (!visited.has(neigh)){

                if(dfs(neigh, target, visited)) return true; 

            }

        }
        return false;
    }
    for(let [x, y] of edges){

        if (mps[x] && mps[y] && dfs(x, y, new Set())){
            return [x, y];
        }

        if (!mps[x]) mps[x] = [];
        if (!mps[y]) mps[y] = [];

        mps[x].push(y);
        mps[y].push(x); 

    }
};
