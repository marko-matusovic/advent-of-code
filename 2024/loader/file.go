package loader

import (
	"fmt"
	"os"
)

func ReadDay(day int, source string) string {
	data, err := os.ReadFile(fmt.Sprintf("input/%s/day_%02d.d", source, day))
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		os.Exit(1)
	}
	return string(data)
}

func WriteDay(day int, data string) {}
