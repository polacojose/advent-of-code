package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
)

func main() {

	bytes, err := getBytes()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Part1:")
	printCheckSum(bytes, 272)

	bytes, err = getBytes()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Part2:")
	printCheckSum(bytes, 35651584)
}

func printCheckSum(bytes []byte, maxLength int) {
	for len(bytes) < maxLength {
		b := make([]byte, len(bytes))
		for i, j := len(bytes)-1, 0; i >= 0; i, j = i-1, j+1 {
			b[j] = bytes[i]
		}

		for i, byte := range b {
			if byte == '1' {
				b[i] = '0'
			} else {
				b[i] = '1'
			}
		}

		bytes = append(bytes, '0')
		bytes = append(bytes, b...)
	}
	bytes = bytes[:maxLength]

	fmt.Println("length:  ", len(bytes))
	fmt.Println("CheckSum:", string(getChecksum(bytes)))
	fmt.Println()
}

func getBytes() ([]byte, error) {
	f, err := os.Open("input.txt")
	if err != nil {
		return []byte{}, err
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)

	for fileScanner.Scan() {
		bytes := []byte(fileScanner.Text())
		return bytes, nil
	}

	return []byte{}, errors.New("unable to read bytes")
}

func getChecksum(bytes []byte) []byte {
	checkSum := append([]byte{}, bytes...)
	for len(checkSum)%2 == 0 {
		newCheckSum := checkSum[:len(checkSum)/2]

		for i, j := 1, 0; i < len(checkSum); i, j = i+2, j+1 {

			if checkSum[i-1] == checkSum[i] {
				newCheckSum[j] = '1'
			} else {
				newCheckSum[j] = '0'
			}
		}

		checkSum = newCheckSum
	}

	return checkSum
}
