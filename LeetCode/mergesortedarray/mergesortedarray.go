package mergesortedarray

import "fmt"

/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n,
representing the number of elements in nums1 and nums2 respectively. Merge nums1 and nums2 into a single array
sorted in non-decreasing order. The final sorted array should not be returned by the function, but instead be
stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements
denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2
has a length of n.

Constraints:
- nums1.length == m + n
- nums2.length == n
- 0 <= m, n <= 200
- 1 <= m + n <= 200
- -10^9 <= nums1[i], nums2[j] <= 10^9
*/
func merge(nums1 []int, m int, nums2 []int, n int) {
	// both empty
	if m == 0 && n == 0 {
		return
	}
	// list 2 empty
	if n == 0 {
		return
	}
	// list 1 empty bot not list 2
	if m == 0 {
		copy(nums1, nums2)
	}
	writeIdx := m + n - 1
	nums2LastElement := n - 1
	nums1LastElement := m - 1
	for writeIdx >= 0 {
		if nums2LastElement < 0 {
			fmt.Println("list two is empty, copying next element of 1")
			nums1[writeIdx] = nums1[nums1LastElement]
			nums1LastElement -= 1
		} else if nums1LastElement < 0 {
			fmt.Println("list one is empty, copying next element of 2")
			nums1[writeIdx] = nums2[nums2LastElement]
			nums2LastElement -= 1
		} else if nums1[nums1LastElement] > nums2[nums2LastElement] {
			fmt.Println("list one element is bigger")
			nums1[writeIdx] = nums1[nums1LastElement]
			nums1LastElement -= 1
		} else {
			fmt.Println("list one element is bigger")
			nums1[writeIdx] = nums2[nums2LastElement]
			nums2LastElement -= 1
		}
		writeIdx -= 1
	}
}
