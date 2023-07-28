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

	//source := "HOHOHO"
	source := "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl"

	transformations := map[string]bool{}

	for fileScanner.Scan() {
		instructionLine := fileScanner.Text()

		results := execute(instructionLine, source)

		for result, _ := range results {
			transformations[result] = false
		}

	}

	fmt.Println(transformations)
	fmt.Println("Size", len(transformations))
}

func execute(instructionLine string, source string) map[string]bool {
	parts := strings.Fields(instructionLine)

	results := map[string]bool{}

	si := 0
	for si < len(source) {
		indexedSource := source[si:]
		i := strings.Index(indexedSource, parts[0])
		if i < 0 {
			break
		}

		si += i

		first := ""
		if si > 0 {
			first = source[:si]
		}

		last := ""
		if si+len(parts[0]) < len(source) {
			last = source[si+len(parts[0]):]
		}

		results[fmt.Sprintf("%s%s%s", first, parts[2], last)] = false

		si++
	}

	return results
}
