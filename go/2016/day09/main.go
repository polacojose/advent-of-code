package main

import (
	"bufio"
	"day09/expander"
	"fmt"
	"os"
	"time"
)

func main() {

	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		ex := expander.NewExpander([]byte(line), true)

		totalBytes := 0
		const buffLength = 1000000
		buff := make([]byte, buffLength)
		start := time.Now()
		for {
			read, err := ex.Expand(&buff)
			if err != nil {
				fmt.Println(err)
				return
			}
			totalBytes += read

			if totalBytes%1000000 == 0 {
				diff := time.Now().UnixMicro() - start.UnixMicro()
				start = time.Now()
				go func() {
					fmt.Printf("%dµs / %d MiByte\n", diff, 1)
					fmt.Println(totalBytes)
				}()
			}

			//fmt.Print(string(buff[:read]))

			if read == 0 {
				break
			}
		}
		fmt.Println()
		fmt.Println(totalBytes)
	}

}
