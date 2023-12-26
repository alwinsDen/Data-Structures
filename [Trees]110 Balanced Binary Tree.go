/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 import "math"
func isBalanced(root *TreeNode) bool {
    _, state:=checker(root)
    return state
}

func checker(root *TreeNode) (int, bool) {
    if root==nil{
        return 0, true
    }
    lld,llb:=checker(root.Left)
    rrd,rrb:=checker(root.Right)
    if !llb||!rrb{
        return 0,false
    }
    if math.Abs(float64(lld-rrd)) > 1 {
        return 0,false
    }
    return int(math.Max(float64(lld), float64(rrd))) + 1, true
}
