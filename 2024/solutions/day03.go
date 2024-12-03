package solutions

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type Day03 struct{}

func (d Day03) Part1(raw string) {
	fmt.Println("Day 3 Part 1")
	r, _ := regexp.Compile(`mul\((\d+),(\d+)\)`)
	sum := 0
	for _, match := range r.FindAllSubmatch([]byte(raw), -1) {
		n1, _ := strconv.Atoi(string(match[1]))
		n2, _ := strconv.Atoi(string(match[2]))
		sum += n1 * n2
	}
	fmt.Println("Solution:", sum)
}

func (d Day03) Part2(raw string) {
	fmt.Println("Day 3 Part 2")

	doParts := make([]string, 0)
	for _, dos := range strings.Split(raw, "do()") {
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
