package solutions

import (
	"fmt"
	"strings"
)

type Day04 struct{}
type input04 struct {
	raw  string
	grid [][]byte
}

func (d Day04) Parse(raw string) input04 {

	grid := make([][]byte, 0)
	for _, line := range strings.Split(raw, "\n") {
		grid = append(grid, []byte(line))
	}

	return input04{raw, grid}
}

func (d Day04) Part1(raw string) {
	fmt.Println("Day 4 Part 1")
	input := d.Parse(raw)
	pattern := []byte("XMAS")
	count := 0

	grid := copyGrid(input.grid)
	for r := 0; r < 4; r++ {
		for y := 0; y < len(grid); y++ {
			for x := 0; x+len(pattern)-1 < len(grid[y]); x++ {
				// horizontal E
				matching := true
				for i := 0; i < len(pattern); i++ {
					if grid[y][x+i] != pattern[i] {
						matching = false
						break
					}
				}
				if matching {
					// fmt.Printf("==================\nr%d x%d y%d %s\n", r, x, y, "E")
					// printGrid(grid)
					count += 1
				}
				// diagonal SE
				if y+len(pattern)-1 >= len(grid) {
					continue
				}
				matching = true
				for i := 0; i < len(pattern); i++ {
					if grid[y+i][x+i] != pattern[i] {
						matching = false
						break
					}
				}
				if matching {
					// fmt.Printf("==================\nr%d x%d y%d %s\n", r, x, y, "SE")
					// printGrid(grid)
					count += 1
				}
			}
		}
		grid = rotateGrid(grid)
	}
	fmt.Println("Solution:", count)
}

func (d Day04) Part2(raw string) {
	fmt.Println("Day 4 Part 2")
	input := d.Parse(raw)
	pattern := []byte("MAS")
	count := 0

	grid := copyGrid(input.grid)
	for r := 0; r < 4; r++ {
		for y := 0; y+len(pattern)-1 < len(grid); y++ {
			for x := 0; x+len(pattern)-1 < len(grid[y]); x++ {
				matching := true
				for i := 0; i < len(pattern); i++ {
					// diagonal \					   diagonal /
					if grid[y+i][x+i] != pattern[i] || grid[y+len(pattern)-1-i][x+i] != pattern[i] {
						matching = false
						break
					}
				}
				if matching {
					// fmt.Printf("==================\nr%d x%d y%d %s\n", r, x, y, "SE")
					// printGrid(grid)
					count += 1
				}
			}
		}
		grid = rotateGrid(grid)
	}
	fmt.Println("Solution:", count)
}

func copyGrid(first [][]byte) [][]byte {
	second := make([][]byte, len(first))
	for i := range second {
		second[i] = make([]byte, len(first[i]))
	}

	for y, line := range first {
		copy(second[y], line)
	}

	return second
}

func rotateGrid(first [][]byte) [][]byte {
	second := make([][]byte, len(first[0]))
	for i := range second {
		second[i] = make([]byte, len(first))
	}

	for y, line := range first {
		for x, ch := range line {
			second[x][len(first)-y-1] = ch
		}
	}

	return second
}

func printGrid(grid [][]byte) {
	for _, line := range grid {
		fmt.Println(string(line))
	}
}
