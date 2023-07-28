package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	dest := "e"
	//source := "HOH"
	source := "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl"

	instructionLines := []string{}

	for fileScanner.Scan() {
		instructionLines = append(instructionLines, fileScanner.Text())
	}

	i := 0
	for source != dest {
		i++
		fmt.Println(source)
		source = execute(&instructionLines, source)
	}
	fmt.Println(source)
	fmt.Println(i)
}

func execute(instructions *[]string, source string) string {
	for _, line := range *instructions {
		parts := strings.Fields(line)

		if strings.Index(source, parts[2]) >= 0 {
			return strings.Replace(source, parts[2], parts[0], 1)
		}

	}
	return source
}
