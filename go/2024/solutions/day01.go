package solutions

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

type Day01 struct{}
type input01 struct {
	Left  []int
	Right []int
}

func (d Day01) Parse(raw string) input01 {
	lines := strings.Split(raw, "\n")
	left := make([]int, len(lines))
	right := make([]int, len(lines))
	for _, line := range lines {
		nums := strings.Split(line, "   ")

		l, _ := strconv.Atoi(nums[0])
		left = append(left, l)

		r, _ := strconv.Atoi(nums[len(nums)-1])
		right = append(right, r)
	}
	return input01{left, right}
}

func (d Day01) Part1(raw string) {
	fmt.Println("Day 1 Part 1")
	input := d.Parse(raw)
	slices.Sort(input.Left)
	slices.Sort(input.Right)

	dist := 0
	for i := 0; i < len(input.Left); i++ {
		dist += int(math.Abs(float64(input.Left[i] - input.Right[i])))
	}
	fmt.Printf("Solution: %d\n", dist)
}

func (d Day01) Part2(raw string) {
	fmt.Println("Day 1 Part 2")
	input := d.Parse(raw)
	total := 0
	for _, valL := range input.Left {
		count := 0
		for _, valR := range input.Right {
			if valL == valR {
				count++
			}
		}
		total += valL * count
	}
	fmt.Printf("Solution: %d\n", total)
}
