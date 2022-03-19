package main

type LRUCache struct {
	size, capacity int
	cache          map[int]*Node
	head, tail     *Node
}

type Node struct {
	key, value int
	prev, next *Node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		size:     capacity,
		capacity: capacity,
		cache:    map[int]*Node{},
		head:     &Node{key: 0, value: 0},
		tail:     &Node{key: 0, value: 0},
	}
}

func (this *LRUCache) Get(key int) int {

}

func (this *LRUCache) Put(key int, value int) {

}

func (this *LRUCache) moveToHead(node *Node) {
	this.removeNode(node)
	node.next = this.head.next
	this.head.next.prev = node
	node.prev = this.head
	this.head.next = node
}

func (this *LRUCache) moveToTail(node *Node) {
	this.removeNode(node)
	node.prev = this.tail.prev
	this.tail.prev.next = node
	node.next = this.tail
	this.tail.prev = node
}

func (this *LRUCache) removeNode(node *Node) {
	node.next.prev = node.prev
	node.prev.next = node.next
}

func main() {
	lruCache := Constructor(2)
	lruCache.Put(1, 1)
	lruCache.Put(2, 2)
	lruCache.Get(1)

}
