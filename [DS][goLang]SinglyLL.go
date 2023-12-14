package main

import "fmt"

type Node struct {
	Data int
	Next *Node
}

// define the head
type List struct {
	Head *Node
}

// define the related functions
func (l *List) add(value int) {
	nnv := &Node{
		Data: value,
	}
	if l.Head == nil {
		l.Head = nnv
		return
	}
	curr := l.Head
	for curr.Next != nil {
		curr = curr.Next
	}
	curr.Next = nnv
}

// define the function to delete
func (l *List) delete(value int) {
	if l.Head == nil {
		return
	}
	if l.Head.Data == value {
		l.Head = l.Head.Next
		return
	}
	//if the value if neither at head but is also present
	curr := l.Head
	for curr.Next != nil && curr.Next.Data != value {
		curr = curr.Next
	}
	if curr.Next != nil {
		curr.Next = curr.Next.Next
	}
}

// print all the values of the linked list by traversing
func (l *List) ppt() {
	curr := l.Head
	for curr != nil {
		fmt.Println(curr.Data)
		curr = curr.Next
	}
}
func main() {
	att := List{}
	att.add(10)
	att.add(9)
	att.add(8)
	att.add(7)
	att.add(6)
	att.delete(6)
	att.ppt()
}
