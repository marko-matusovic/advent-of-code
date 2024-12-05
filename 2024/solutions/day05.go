package solutions

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type Day05 struct{}
type input05 struct {
	raw    string
	rules  []rule
	prints [][]int
}
type rule struct{ a, b int }

func (d Day05) Parse(raw string) input05 {
	parts := strings.Split(raw, "\n\n")
	rawRules := strings.Split(parts[0], "\n")
	rawPrints := strings.Split(parts[1], "\n")

	rules := make([]rule, len(rawRules))
	for i, r := range rawRules {
		parts := strings.Split(r, "|")
		a, _ := strconv.Atoi(parts[0])
		b, _ := strconv.Atoi(parts[1])
		rules[i] = rule{a, b}
	}
	prints := make([][]int, len(rawPrints))
	for i, p := range rawPrints {
		parts := strings.Split(p, ",")
		prints[i] = make([]int, len(parts))
		for j, sn := range parts {
			n, _ := strconv.Atoi(sn)
			prints[i][j] = n
		}
	}
	return input05{raw, rules, prints}
}

func (d Day05) Part1(raw string) {
	fmt.Println("Day 5 Part 1")
	input := d.Parse(raw)
	sum := 0

printLoop:
	for _, print := range input.prints {
		for _, rule := range input.rules {
			if posB := slices.IndexFunc(print, match(rule.b)); posB >= 0 {
				if posA := slices.IndexFunc(print, match(rule.a)); posA >= 0 && posB < posA {
					continue printLoop
				}
			}
		}
		// all rules pass
		sum += print[int(len(print)/2)]
	}

	fmt.Println("Solution:", sum)
}

func (d Day05) Part2(raw string) {
	fmt.Println("Day 5 Part 2")
	input := d.Parse(raw)
	sum := 0

	for _, print := range input.prints {
		include := false
		check := true
		for check {
			check = false
			for _, rule := range input.rules {
				if posB := slices.IndexFunc(print, match(rule.b)); posB >= 0 {
					if posA := slices.IndexFunc(print, match(rule.a)); posA >= 0 && posB < posA {
						include = true
						check = true
						print[posA], print[posB] = print[posB], print[posA]
					}
				}
			}
		}
		if include {
			sum += print[int(len(print)/2)]
		}
	}

	fmt.Println("Solution:", sum)
}

func match(n int) func(int) bool {
	return func(element int) bool {
		return element == n
	}
}
