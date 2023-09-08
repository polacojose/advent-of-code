package main

import (
	"day09/expander"
	"fmt"
	"testing"
)

func BenchmarkExtract(b *testing.B) {
	line := []byte("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")

	for i := 0; i < b.N; i++ {
		ex := expander.NewExpander(line, true)

		totalBytes := 0
		buff := make([]byte, 10000)
		for {
			read, err := ex.Expand(&buff)
			if err != nil {
				fmt.Println(err)
			}

			totalBytes += read

			if totalBytes%1000000 == 0 {
				fmt.Println(totalBytes)
			}

			if read == 0 {
				break
			}
		}
	}
}
