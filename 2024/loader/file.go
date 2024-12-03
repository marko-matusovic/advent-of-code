package loader

import (
	"fmt"
	"os"
)

func ReadDay(day int) string {
	data, err := os.ReadFile(fmt.Sprintf("input/my/day_%02d.d", day))
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		os.Exit(1)
	}
	return string(data)
}

func WriteDay(day int, data string) {}
