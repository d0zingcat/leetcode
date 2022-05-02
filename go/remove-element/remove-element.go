package remove_element

func removeElement(nums []int, val int) int {
	var left, right int
	for ; right < len(nums); right++ {
		if nums[right] != val {
			nums[left] = nums[right]
			left++
		}
	}
	return left
}

func removeElement2(nums []int, val int) int {
	var left, right = 0, len(nums)
	for left < right {
		if nums[left] == val {
			nums[left] = nums[right-1]
			right--
		} else {
			left++
		}
	}
	return left
}
