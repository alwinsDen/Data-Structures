//DFS

class TreeNode {
    constructor(value) {
        this.value = value,
        this.children = [];
    }
    
    addChild(node){
        this.children.push(node);
    }
    
}

//DFS algorithm
function depthFirstSearch(root, target){
    if (root.value == target) return true;
    for (let child of root.children) {
        if (depthFirstSearch(child, target)) {
            return true;
        }
    }
    return false;
}


const root = new TreeNode(1)
const child1 = new TreeNode(2)
const child2 = new TreeNode(3)

root.addChild(child1);
root.addChild(child2);

console.log(depthFirstSearch(root, 32))
