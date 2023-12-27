import "math"
type ssm struct {
    Val int
    Cnt int
    Acc float64
}
type sorter []ssm
func(a sorter) Len () int { return len(a) }
func(a sorter) Swap (i, j int) { a[i],a[j]=a[j],a[i] }
func(a sorter) Less (i, j int) bool { return a[i].Cnt < a[j].Cnt }
func closestValue(root *TreeNode, target float64) int {
    var rrt []ssm
    helper(root,&rrt,target)
    sort.Sort(sorter(rrt))
    if len(rrt)>1 {
        if rrt[0].Acc==rrt[1].Acc {
            return rrt[1].Val
        }
    }
    return rrt[0].Val
}
func helper(root *TreeNode,rrt *[]ssm,target float64)(*TreeNode,*[]ssm,float64){
    if root==nil{
        return nil,rrt,target
    }
    *rrt=append(*rrt,ssm{root.Val,int( math.Abs(float64( int (math.Round(target))-root.Val ))),math.Abs(target - float64(root.Val))})
    helper(root.Left,rrt,target)
    helper(root.Right,rrt,target)
    return root,rrt,target
}
