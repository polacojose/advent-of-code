package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Instruction string

const (
	hlf Instruction = "hlf"
	tpl Instruction = "tpl"
	inc Instruction = "inc"
	jmp Instruction = "jmp"
	jie Instruction = "jie"
	jio Instruction = "jio"
)

var REGISTERS map[string]int

func main() {

	file, _ := os.Open("input.txt")
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	instructionLines := []string{}
	for fileScanner.Scan() {
		instructionLines = append(instructionLines, fileScanner.Text())
	}

	fmt.Println("a starts 0")
	REGISTERS = make(map[string]int)
	//fmt.Println(REGISTERS)
	for getRegister("ip") < len(instructionLines) {
		ip := getRegister("ip")
		instructionLine := instructionLines[ip]
		//fmt.Printf("%d: %s\n", ip, instructionLine)
		executeInstructionline(instructionLine)
		//fmt.Println(REGISTERS)
	}
	fmt.Println(REGISTERS)

	fmt.Println("a starts 1")
	REGISTERS = make(map[string]int)
	setRegister("a", 1)
	//fmt.Println(REGISTERS)
	for getRegister("ip") < len(instructionLines) {
		ip := getRegister("ip")
		instructionLine := instructionLines[ip]
		//fmt.Printf("%d: %s\n", ip, instructionLine)
		executeInstructionline(instructionLine)
		//fmt.Println(REGISTERS)
	}
	fmt.Println(REGISTERS)
}

func offsetIP(offset int) {
	ip := getRegister("ip")
	setRegister("ip", ip+offset)
}

func setRegister(r string, val int) {
	REGISTERS[r] = val
}

func getRegister(r string) int {
	if val, ok := REGISTERS[r]; ok {
		return val
	}

	REGISTERS[r] = 0
	return 0
}

func getParamValue(param string) int {
	intVal, err := strconv.Atoi(param)
	if err == nil {
		return intVal
	}

	return getRegister(param)

}

func executeInstructionline(line string) {
	lineParts := strings.Fields(strings.ReplaceAll(line, ",", ""))
	paramA := lineParts[1]

	switch Instruction(lineParts[0]) {
	case hlf:
		val := getRegister(paramA)
		setRegister(paramA, val/2)
		offsetIP(1)
	case tpl:
		val := getRegister(paramA)
		setRegister(paramA, val*3)
		offsetIP(1)
	case inc:
		val := getRegister(paramA)
		setRegister(paramA, val+1)
		offsetIP(1)
	case jmp:
		val := getParamValue(paramA)
		offsetIP(val)
	case jie:
		val := getParamValue(paramA)
		if val%2 == 0 {
			paramB := lineParts[2]
			val = getParamValue(paramB)
			offsetIP(val)
		} else {
			offsetIP(1)
		}
	case jio:
		val := getParamValue(paramA)
		if val == 1 {
			paramB := lineParts[2]
			val = getParamValue(paramB)
			offsetIP(val)
		} else {
			offsetIP(1)
		}
	default:
		fmt.Println("Unknown Instruction!", lineParts[0])
	}
}
