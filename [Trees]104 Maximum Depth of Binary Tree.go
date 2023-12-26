/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
import "math"
func maxDepth(root *TreeNode) int {
    if root==nil{
        return 0
    }
    lld:=maxDepth(root.Left)
    rrd:=maxDepth(root.Right)
    return int(math.Max(float64(lld),float64(rrd)))+1
}
