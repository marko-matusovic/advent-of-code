package days

import (
	"aoc_2024/loader"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type input03 struct {
	Data []byte
}

func loadInput03() input03 {
	return input03{[]byte(loader.ReadDay(3))}
}

func Day03Part1() {
	fmt.Println("Day 3 Part 1")
	input := loadInput03().Data
	r, _ := regexp.Compile(`mul\((\d+),(\d+)\)`)
	sum := 0
	for _, match := range r.FindAllSubmatch(input, -1) {
		n1, _ := strconv.Atoi(string(match[1]))
		n2, _ := strconv.Atoi(string(match[2]))
		sum += n1 * n2
	}
	fmt.Println("Solution:", sum)
}

func Day03Part2() {
	fmt.Println("Day 3 Part 2")
	input := loadInput03().Data

	doParts := make([]string, 0)
	for _, dos := range strings.Split(string(input), "do()") {
		doParts = append(doParts, strings.Split(dos, "don't()")[0])
	}
	doInput := strings.Join(doParts, " ")

	r, _ := regexp.Compile(`mul\((\d+),(\d+)\)`)
	sum := 0
	for _, match := range r.FindAllSubmatch([]byte(doInput), -1) {
		n1, _ := strconv.Atoi(string(match[1]))
		n2, _ := strconv.Atoi(string(match[2]))
		sum += n1 * n2
	}
	fmt.Println("Solution:", sum)
}
