func isSameTree(p *TreeNode, q *TreeNode) bool {
    sts:=helper(p,q)
    return sts
}
func helper(p *TreeNode,q *TreeNode) bool {
    if p==nil&&q==nil{
        return true
    }
    if p==nil||q==nil {
        return false
    }
    //make sure it is True thougout.
    return (p.Val==q.Val)&&helper(p.Left,q.Left)&&helper(p.Right,q.Right)
}
