package main

import "fmt"

func main() {
	arr := []int{1, 2, 3, 4, 5}
	prefixSum := make([]int, len(arr))
	prefixSum[0] = arr[0]
	for i := 1; i < len(arr); i++ {
		prefixSum[i] = prefixSum[i-1] + arr[i]
	}
	L, R := 2, 3
	sum := prefixSum[R]
	sum -= prefixSum[L-1]
	fmt.Printf("Sum of elements from index %d to %d is %d\n", L, R, sum)
}
