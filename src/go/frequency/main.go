package main

import (
	"fmt"
	"unicode"
)

func frequencyAnalysis(text string) map[rune]int {
	frequency := make(map[rune]int)
	for _, char := range text {
		if unicode.IsLetter(char) {
			frequency[char]++
		}
	}
	return frequency
}

func main() {
	text := "WNEPMLXDHXZEPEDHEZREATDRXREOTLGEKVNHTZREKELYOYRXREEMXJXTLTDFTRXITVDXRX"
	frequencies := frequencyAnalysis(text)

	fmt.Println("Letter Frequencies:")
	for char, freq := range frequencies {
		fmt.Printf("%c: %d\n", char, freq)
	}
}
