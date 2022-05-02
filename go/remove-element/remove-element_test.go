package remove_element

import (
	"fmt"
	"testing"
)

type question1 struct {
	para1
	ans1
}

// para 是参数
// one 代表第一个参数
type para1 struct {
	nums   []int
	target int
}

// ans 是答案
// one 代表第一个答案
type ans1 struct {
	one []int
}

func TestSolution1(t *testing.T) {
	qs := []question1{
		{
			para1{[]int{3, 2, 2, 3}, 3},
			ans1{[]int{2}},
		},
		{
			para1{[]int{0, 1, 2, 2, 3, 0, 4, 2}, 2},
			ans1{[]int{5}},
		},
		{
			para1{[]int{1, 1, 1}, 1},
			ans1{[]int{0}},
		},
	}
	fmt.Printf("------------------------Leetcode Problem 1------------------------\n")

	for _, q := range qs {
		_, p := q.ans1, q.para1
		fmt.Printf("【input】:%v       【output】:%v\n", p, removeElement(p.nums, p.target))
	}
	fmt.Printf("\n\n\n")
}
