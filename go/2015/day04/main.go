package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"strconv"
)

func main() {

	const secret = "iwrupvqb"

	firstFive := 0
	firstSix := 0

	for i := 0; ; i++ {
		h := md5.New()
		s := secret + strconv.Itoa(i)
		io.WriteString(h, s)
		hex := fmt.Sprintf("%x\n", h.Sum(nil))
		if firstFive == 0 && hex[:5] == "00000" {
			firstFive = i
		}
		if firstSix == 0 && hex[:6] == "000000" {
			firstSix = i
			break
		}
	}
	fmt.Println("First five:", firstFive)
	fmt.Println("First six:", firstSix)
}
