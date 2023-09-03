package main

import (
	"fmt"
)

const startCode = 20_151_125

func main() {

	var (
		r       = 1
		c       = 1
		targetR = 3010
		targetC = 3019
		code    = startCode
	)

	for {

		r--
		c++

		if r == 0 {
			r = c
			c = 1
		}

		code = nextCode(code)

		if r == targetR && c == targetC {
			fmt.Println(r, c, code)
			break
		}
	}
}

func nextCode(code int) int {
	return code * 252_533 % 3_355_4393
}
