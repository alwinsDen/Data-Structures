/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
import "math"
func diameterOfBinaryTree(root *TreeNode) int {
    dia:=0
    checker(root,&dia)
    return dia
}
func checker(root *TreeNode,dia *int) int {
    if root==nil{
        return 0
    }
    lld:=checker(root.Left,dia)
    rrd:=checker(root.Right,dia)
    *dia = int(math.Max(float64(*dia),float64(lld+rrd)))
    return int(math.Max(float64(lld),float64(rrd)))+1
}
