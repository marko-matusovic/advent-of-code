package solutions

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Day02 struct{}
type input02 struct {
	Reports [][]int
}

func (d Day02) Parse(raw string) input02 {
	lines := strings.Split(raw, "\n")
	reports := make([][]int, len(lines))
	for i, line := range lines {
		nums := make([]int, 0)
		for _, num := range strings.Split(line, " ") {
			n, _ := strconv.Atoi(num)
			nums = append(nums, n)
		}
		reports[i] = nums
	}
	return input02{reports}
}

func (d Day02) Part1(raw string) {
	fmt.Println("Day 2 Part 1")
	input := d.Parse(raw)

	count := 0
	for _, report := range input.Reports {
		if safe(report) {
			count++
		}
	}

	fmt.Printf("Solution: %d\n", count)
}

func (d Day02) Part2(raw string) {
	fmt.Println("Day 2 Part 2")
	input := d.Parse(raw)

	count := 0
	for _, report := range input.Reports {
		if safe(report[1:]) || safe(report[:len(report)-1]) {
			count += 1
			continue
		}
		for i := 1; i+1 < len(report); i++ {
			short := make([]int, 0)
			short = append(short, report[:i]...)
			short = append(short, report[i+1:]...)
			if safe(short) {
				count += 1
				break
			}
		}
	}

	fmt.Printf("Solution: %d\n", count)
}

func safe(report []int) bool {
	last := report[0]
	inc := report[0] < report[1]
	for i := 1; i < len(report); i++ {
		n := report[i]
		if last == n || last < n != inc || 3 < math.Abs(float64(n-last)) {
			return false
		}
		last = n
	}
	return true
}
