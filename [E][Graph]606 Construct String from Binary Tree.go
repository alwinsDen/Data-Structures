/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
import "fmt"

func tree2str(root *TreeNode) string {
    if root==nil {
        return ""
    }
    ans := strconv.Itoa(root.Val)
    if root.Left!=nil || root.Right!=nil {
        ans += "(" + tree2str(root.Left) + ")"
    }
    if root.Right!=nil {
        ans += "(" + tree2str(root.Right) + ")"
    }
    return ans
}
