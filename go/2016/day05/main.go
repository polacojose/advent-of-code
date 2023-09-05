package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"strconv"
)

func main() {

	standard()
	positional()
}

func positional() {
	chars := 0
	posPassword := make([]byte, 8)
	alloc := make(map[int]bool, 8)
	for i := 0; ; i++ {
		h := md5.New()
		io.WriteString(h, "wtnhxymk"+strconv.Itoa(i))
		hashString := fmt.Sprintf("%x", h.Sum(nil))
		if hashString[:5] == "00000" {
			pos, err := strconv.Atoi(string(hashString[5]))
			if err != nil {
				continue
			}

			if pos >= 8 {
				continue
			}

			if _, ok := alloc[pos]; !ok {
				alloc[pos] = true
				posPassword[pos] = hashString[6]
				chars++
			} else {
				continue
			}

			fmt.Printf("%d%% ", chars*100/8)

			if chars >= 8 {
				break
			}
		}
	}
	fmt.Println()
	fmt.Println("Positional Password:", string(posPassword))
}

func standard() {
	password := []byte{}
	for i := 0; ; i++ {
		h := md5.New()
		io.WriteString(h, "wtnhxymk"+strconv.Itoa(i))
		hashString := fmt.Sprintf("%x", h.Sum(nil))
		if hashString[:5] == "00000" {
			password = append(password, hashString[5])
			fmt.Printf("%d%% ", len(password)*100/8)
			if len(password) >= 8 {
				break
			}
		}
	}
	fmt.Println()
	fmt.Println("Standard Password:", string(password))
}
