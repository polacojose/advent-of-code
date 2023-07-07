package lib

import (
	"errors"
	"strconv"
	"strings"
)

var Wires map[string]uint16

func init() {
	Wires = make(map[string]uint16)
}

func ExecuteInstruction(instructionLine string) bool {
	parts := strings.Fields(instructionLine)

	if len(parts) == 3 {
		return executeAssign(instructionLine)
	} else if strings.Contains(instructionLine, "AND") {
		return executeAND(instructionLine)
	} else if strings.Contains(instructionLine, "OR") {
		return executeOR(instructionLine)
	} else if strings.Contains(instructionLine, "LSHIFT") {
		return executeLSHIFT(instructionLine)
	} else if strings.Contains(instructionLine, "RSHIFT") {
		return executeRSHIFT(instructionLine)
	} else if strings.Contains(instructionLine, "NOT") {
		return executeNOT(instructionLine)
	}

	return false
}

func getOperand(operand string) (uint16, error) {
	value, err := strconv.Atoi(operand)
	if err != nil {
		_, ok := Wires[operand]
		if ok {
			return Wires[operand], nil
		}

		return 0, errors.New("Operand Missing")
	}

	return uint16(value), nil
}

func executeNOT(instructionLine string) bool {
	instructionParts := strings.Fields(instructionLine)
	operand, err := getOperand(instructionParts[1])
	if err != nil {
		return false
	}

	dest := instructionParts[3]

	Wires[dest] = ^operand

	return true
}

func executeRSHIFT(instructionLine string) bool {
	instructionParts := strings.Fields(instructionLine)
	operand_a, err_a := getOperand(instructionParts[0])
	operand_b, err_b := getOperand(instructionParts[2])

	if err_a != nil || err_b != nil {
		return false
	}

	dest := instructionParts[4]

	Wires[dest] = operand_a >> operand_b

	return true
}

func executeLSHIFT(instructionLine string) bool {
	instructionParts := strings.Fields(instructionLine)
	operand_a, err_a := getOperand(instructionParts[0])
	operand_b, err_b := getOperand(instructionParts[2])

	if err_a != nil || err_b != nil {
		return false
	}

	dest := instructionParts[4]

	Wires[dest] = operand_a << operand_b

	return true
}

func executeOR(instructionLine string) bool {
	instructionParts := strings.Fields(instructionLine)
	operand_a, err_a := getOperand(instructionParts[0])
	operand_b, err_b := getOperand(instructionParts[2])

	if err_a != nil || err_b != nil {
		return false
	}

	dest := instructionParts[4]

	Wires[dest] = operand_a | operand_b

	return true
}

func executeAND(instructionLine string) bool {
	instructionParts := strings.Fields(instructionLine)
	operand_a, err_a := getOperand(instructionParts[0])
	operand_b, err_b := getOperand(instructionParts[2])

	if err_a != nil || err_b != nil {
		return false
	}

	dest := instructionParts[4]

	Wires[dest] = operand_a & operand_b

	return true
}

func executeAssign(instructionLine string) bool {
	instructionParts := strings.Fields(instructionLine)
	operand, err := getOperand(instructionParts[0])

	if err != nil {
		return false
	}

	dest := instructionParts[2]

	Wires[dest] = operand

	return true
}
