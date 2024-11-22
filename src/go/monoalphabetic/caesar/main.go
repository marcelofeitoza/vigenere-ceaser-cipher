package caesar

import (
	"fmt"
	"strings"
	"unicode"
)

func CeasarEncript(text string, shift int) string {
	var result = ""

	for _, char := range text {
		if unicode.IsLetter(char) {
			base := 'A'
			if unicode.IsLower(char) {
				base = 'a'
			}

			newLetter := (char-base+int32(shift))%26 + base
			result += string(newLetter)
		}
	}
	return result
}

func CeasarDecript(text string, shift int) string {
	var result = ""

	for _, char := range text {
		if unicode.IsLetter(char) {
			base := 'A'
			if unicode.IsLower(char) {
				base = 'a'
			}

			newLetter := (char-base-int32(shift))%26 + base
			result += string(newLetter)
		}
	}
	return result
}

func VictorEncryptCaesar(text string, shift int) {
	alphabet := [26]string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}
	shift = shift % 26

	var result strings.Builder

	for _, char := range text {
		lowerChar := strings.ToLower(string(char))
		index := -1
		for i, letter := range alphabet {
			if letter == lowerChar {
				index = i
				break
			}
		}

		if index != -1 {
			newIndex := (index + shift) % 26
			newChar := alphabet[newIndex]

			if char >= 'A' && char <= 'Z' {
				newChar = strings.ToUpper(newChar)
			}
			result.WriteString(newChar)
		} else {
			result.WriteString(string(char))
		}
	}

	fmt.Println("Mensagem Cifrada:", result.String())
}

func VictorDecryptCaesar(text string, shift int) {
	alphabet := [26]string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}
	shift = shift * (-1)
	shift = shift % -26

	var result strings.Builder

	for _, char := range text {
		lowerChar := strings.ToLower(string(char))
		index := -1
		for i, letter := range alphabet {
			if letter == lowerChar {
				index = i
				break
			}
		}

		if index != -1 {
			newIndex := (index + shift) % 26
			newChar := alphabet[newIndex]

			if char >= 'A' && char <= 'Z' {
				newChar = strings.ToUpper(newChar)
			}
			result.WriteString(newChar)
		} else {
			result.WriteString(string(char))
		}
	}

	fmt.Println("Mensagem Decifrada:", result.String())
}
