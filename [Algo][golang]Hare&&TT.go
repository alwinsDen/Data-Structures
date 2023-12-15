package main

import (
	"fmt"
)

type ListNode struct {
	Value int
	Next  *ListNode
}

// function to detect cycle using Floyd's Tortoise and Hare algorithm
func detectCycle(head *ListNode) *ListNode {
	tortoise, hare := head, head

	// Move tortoise by 1 and hare by 2 steps
	for hare != nil && hare.Next != nil {
		tortoise = tortoise.Next
		hare = hare.Next.Next

		// Cycle detected
		if tortoise == hare {
			// Find the start of the cycle
			tortoise = head
			for tortoise != hare {
				tortoise = tortoise.Next
				hare = hare.Next
			}
			return tortoise
		}
	}

	// No cycle found
	return nil
}

func main() {
	// Example: Creating a linked list with a cycle
	head := &ListNode{Value: 1}
	head.Next = &ListNode{Value: 2}
	head.Next.Next = &ListNode{Value: 3}
	head.Next.Next.Next = &ListNode{Value: 4}
	head.Next.Next.Next.Next = head.Next // creating a cycle

	cycleNode := detectCycle(head)
	if cycleNode != nil {
		fmt.Printf("Cycle detected starting at node with value: %d\n", cycleNode.Value)
	} else {
		fmt.Println("No cycle detected")
	}
}
