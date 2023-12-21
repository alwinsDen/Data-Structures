func copyRandomList(head *Node) *Node {
    hmp:=make(map[*Node]*Node)
    ll:=head
    for ll!=nil{
        hmp[ll]=&Node{Val: ll.Val}
        ll=ll.Next
    }
    mm:=head
    for mm!=nil{
        kk:=hmp[mm]
        kk.Next=hmp[mm.Next]
        kk.Random=hmp[mm.Random]
        mm=mm.Next
    }
    return hmp[head]
}
