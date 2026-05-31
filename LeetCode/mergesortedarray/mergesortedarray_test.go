package mergesortedarray

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type testCase struct {
	name      string
	slice1    []int
	slice1Len int
	slice2    []int
	slice2Len int
	expected  []int
}

func TestMerge(t *testing.T) {
	testCases := []testCase{
		{
			name:     "empty lists",
			slice1:   []int{},
			slice2:   []int{},
			expected: []int{},
		},
		{
			name:      "single list single digit",
			slice1:    []int{1},
			slice1Len: 1,
			slice2:    []int{},
			expected:  []int{1},
		},
		{
			name:      "single list single digit second list",
			slice1:    []int{0},
			slice2:    []int{1},
			slice2Len: 1,
			expected:  []int{1},
		},
		{
			name:      "two lists",
			slice1:    []int{0, 2, 0},
			slice1Len: 2,
			slice2:    []int{1},
			slice2Len: 1,
			expected:  []int{0, 1, 2},
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			merge(tc.slice1, tc.slice1Len, tc.slice2, tc.slice2Len)

			assert.Equal(t, tc.expected, tc.slice1)
		})
	}
}
