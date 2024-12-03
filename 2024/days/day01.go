package days

import (
	"aoc_2024/loader"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

type input01 struct {
	Left  []int
	Right []int
}

func loadInput01() input01 {
	lines := strings.Split(loader.ReadDay(1), "\n")
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

func Day01Part1() {
	fmt.Println("Day 1 Part 1")
	input := loadInput01()
	slices.Sort(input.Left)
	slices.Sort(input.Right)

	dist := 0
	for i := 0; i < len(input.Left); i++ {
		dist += int(math.Abs(float64(input.Left[i] - input.Right[i])))
	}
	fmt.Printf("Solution: %d\n", dist)
}

func Day01Part2() {
	fmt.Println("Day 1 Part 2")
	input := loadInput01()
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
