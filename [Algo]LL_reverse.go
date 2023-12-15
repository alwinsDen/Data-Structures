package main

import "fmt"

type Node struct {
	Value int
	Next  *Node
}

func (sf *Node) ReverseNodes() *Node {
	var rev *Node
	cc := sf
	for cc != nil {
		//cache the value from 2nd element
		rr := cc.Next
		//set the next of 1st element as rev
		cc.Next = rev
		//set the current reverse cc as rev
		rev = cc
		//continue cc as using .Next values
		cc = rr
	}
	return rev
}

func main() {
	tt := &Node{Value: 10}
	tt.Next = &Node{Value: 20}
	tt.Next.Next = &Node{Value: 30}
	jj := tt.ReverseNodes()
	cc := jj
	for cc != nil {
		fmt.Println(cc.Value)
		cc = cc.Next
	}
}
