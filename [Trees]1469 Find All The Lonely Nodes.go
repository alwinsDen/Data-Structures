func getLonelyNodes(root *TreeNode) []int {
    var ssm []int
    helper(root,&ssm)
    fmt.Println(ssm)
    return ssm
}
func helper(root *TreeNode,ssm *[]int)(*TreeNode,*[]int){
    if root==nil {
        return nil,ssm
    }
    lld,_:=helper(root.Left,ssm)
    rrd,_:=helper(root.Right,ssm)
    if lld!=nil && rrd==nil {
        *ssm=append(*ssm,lld.Val)
    } else if lld==nil && rrd!=nil {
        *ssm=append(*ssm,rrd.Val)
    }
    return root, ssm
}
