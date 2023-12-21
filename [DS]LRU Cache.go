package main

import "fmt"

type Node struct {
	key, value int
	prev, next *Node
}

type LRUCache struct {
	capacity int
	size     int
	cache    map[int]*Node
	head     *Node
	tail     *Node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		cache:    make(map[int]*Node),
		head:     &Node{},
		tail:     &Node{},
	}
}

func (this *LRUCache) Get(key int) int {
	if node, ok := this.cache[key]; ok {
		this.moveToFront(node)
		return node.value
	}
	return -1
}

func (this *LRUCache) Put(key int, value int) {
	if node, ok := this.cache[key]; ok {
		node.value = value
		this.moveToFront(node)
		return
	}
	newNode := &Node{key: key, value: value}
	this.cache[key] = newNode
	this.addToFront(newNode)

	if this.size > this.capacity {
		removed := this.removeLRUItem()
		delete(this.cache, removed.key)
	}
}

func (this *LRUCache) addToFront(node *Node) {
	node.prev = this.head
	node.next = this.head.next

	if this.head.next != nil {
		this.head.next.prev = node
	}
	this.head.next = node

	if this.tail.prev == nil {
		this.tail.prev = node
	}

	this.size++
}

func (this *LRUCache) removeLRUItem() *Node {
	removed := this.tail.prev
	if removed != nil {
		removed.prev.next = this.tail
		this.tail.prev = removed.prev
		removed.prev = nil
		removed.next = nil
		this.size--
	}
	return removed
}

func (this *LRUCache) moveToFront(node *Node) {
	if node.prev != nil {
		node.prev.next = node.next
	}
	if node.next != nil {
		node.next.prev = node.prev
	}

	node.prev = this.head
	node.next = this.head.next

	if this.head.next != nil {
		this.head.next.prev = node
	}
	this.head.next = node

	if this.tail.prev == nil {
		this.tail.prev = node
	}
}

func main() {
	lru := Constructor(2)
	lru.Put(1, 1)
	lru.Put(2, 2)
	fmt.Println(lru.Get(1))   // returns 1
	lru.Put(3, 3)      // evicts key 2
	fmt.Println(lru.Get(2))  // returns -1 (not found)
	lru.Put(4, 4)      // evicts key 1
	fmt.Println(lru.Get(1))  // returns -1 (not found)
	fmt.Println(lru.Get(3))  // returns 3
	fmt.Println(lru.Get(4))  // returns 4
}
