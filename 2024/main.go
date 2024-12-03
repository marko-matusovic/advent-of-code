package main

import (
	"aoc_2024/loader"
	"aoc_2024/solutions"
	"os"
	"strconv"
)

func main() {
	dayN, _ := strconv.Atoi(os.Args[1])
	solution := map[int]solutions.Day{
		1:  solutions.Day01{},
		2:  solutions.Day02{},
		3:  solutions.Day03{},
		4:  solutions.Day04{},
		5:  solutions.Day05{},
		6:  solutions.Day06{},
		7:  solutions.Day07{},
		8:  solutions.Day08{},
		9:  solutions.Day09{},
		10: solutions.Day10{},
		11: solutions.Day11{},
		12: solutions.Day12{},
		13: solutions.Day13{},
		14: solutions.Day14{},
		15: solutions.Day15{},
		16: solutions.Day16{},
		17: solutions.Day17{},
		18: solutions.Day18{},
		19: solutions.Day19{},
		20: solutions.Day20{},
		21: solutions.Day21{},
		22: solutions.Day22{},
		23: solutions.Day23{},
		24: solutions.Day24{},
		25: solutions.Day25{},
	}[dayN]

	solution.Part1(loader.ReadDay(dayN))
	solution.Part2(loader.ReadDay(dayN))
}
