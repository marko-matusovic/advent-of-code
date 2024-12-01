package days

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Input struct {
	Left  []int
	Right []int
}

func loadInput() Input {

	data, err := os.ReadFile("input/my/day_01.d")

	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		os.Exit(1)
	}

	lines := strings.Split(string(data), "\n")
	left := make([]int, len(lines))
	right := make([]int, len(lines))
	for _, line := range lines {
		nums := strings.Split(line, "   ")

		l, _ := strconv.Atoi(nums[0])
		left = append(left, l)

		r, _ := strconv.Atoi(nums[len(nums)-1])
		right = append(right, r)
	}
	return Input{left, right}
}

func Day01Part1() {
	fmt.Println("Day 1 Part 1")
	input := loadInput()
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
	input := loadInput()
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
