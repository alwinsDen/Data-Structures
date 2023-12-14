package main

import "fmt"

// Node node structs
type Node struct {
	Value int
	Next  *Node
	Prev  *Node
}

// Dll DLL data-s
type Dll struct {
	Head *Node
	Tail *Node
}

// CreateDLL add a new Node DS
func CreateDLL(vls int) *Node {
	return &Node{Value: vls}
}

// append value to the end of the DLL
func (slf *Dll) pushBack(vls int) {
	dll := CreateDLL(vls)
	if slf.Head == nil {
		slf.Head = dll
		slf.Tail = dll
		return
	}
	slf.Tail.Next = dll
	dll.Prev = slf.Tail
	slf.Tail = dll
}

// prepend the value onto the start of the DLL
func (slf *Dll) pushFront(vls int) {
	dll := CreateDLL(vls)
	if slf.Head == nil {
		slf.Head = dll
		slf.Tail = dll
		return
	}
	slf.Head.Prev = dll
	dll.Next = slf.Head
	slf.Head = dll
}

// deleting the value from the Dll
func (slf *Dll) delete(value int) {
	if slf.Head == nil {
		return
	}
	//now start the search
	curr := slf.Head
	for curr != nil {
		if curr.Value == value {
			if curr.Prev != nil {
				curr.Prev.Next = curr.Next
			} else {
				slf.Head = curr.Next
			}
			if curr.Next != nil {
				curr.Next.Prev = curr.Prev
			} else {
				slf.Tail = curr.Prev
			}
			return
		}
		curr = curr.Next
	}
}

// Fprint print forwards
func (slf *Dll) Fprint() {
	dll := slf.Head
	for dll != nil {
		fmt.Println(dll.Value)
		dll = dll.Next
	}
}

func main() {
	dll := &Dll{}
	dll.pushBack(10)
	dll.pushFront(20)
	dll.delete(10)
	dll.Fprint()
}
