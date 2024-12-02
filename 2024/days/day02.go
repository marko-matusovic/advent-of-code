package days

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type input02 struct {
	Reports [][]int
}

func loadInput02() input02 {

	data, err := os.ReadFile("input/my/day_02.d")

	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		os.Exit(1)
	}
	lines := strings.Split(string(data), "\n")

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

func Day02Part1() {
	fmt.Println("Day 2 Part 1")
	input := loadInput02()

	count := 0
	for _, report := range input.Reports {
		if safe(report) {
			count++
		}
	}

	fmt.Printf("Solution: %d\n", count)
}

func Day02Part2() {
	fmt.Println("Day 2 Part 2")
	input := loadInput02()

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
