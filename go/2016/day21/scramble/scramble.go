package scramble

import (
	"errors"
	"strconv"
	"strings"
)

type InstType string

const (
	swapPos  = "swapPos"
	swapLet  = "swapLet"
	rev      = "rev"
	mov      = "mov"
	rotLeft  = "rotLeft"
	rotRight = "rotRight"
	rotLet   = "rotLet"
	invalid  = "invalid"
)

type Scrambler struct {
	subject []byte
}

func New(subject []byte) Scrambler {
	return Scrambler{subject}
}

func (s *Scrambler) Out() []byte {
	return s.subject
}

func (s *Scrambler) Execute(inst string) error {
	instFields := strings.Fields(inst)
	inst_type, err := getType(instFields)
	if err != nil {
		return err
	}

	switch inst_type {
	case swapPos:
		a, err := strconv.Atoi(instFields[2])
		if err != nil {
			return err
		}
		b, err := strconv.Atoi(instFields[5])
		if err != nil {
			return err
		}
		s.swapPos(a, b)
	case swapLet:
		a := instFields[2]
		b := instFields[5]
		s.swapLetter(a, b)
	case rev:
		a, err := strconv.Atoi(instFields[2])
		if err != nil {
			return err
		}
		b, err := strconv.Atoi(instFields[4])
		if err != nil {
			return err
		}
		s.rev(a, b)
	case mov:
		a, err := strconv.Atoi(instFields[2])
		if err != nil {
			return err
		}
		b, err := strconv.Atoi(instFields[5])
		if err != nil {
			return err
		}
		s.mov(a, b)
	case rotLeft:
		a, err := strconv.Atoi(instFields[2])
		if err != nil {
			return err
		}
		s.rotLeft(a)
	case rotRight:
		a, err := strconv.Atoi(instFields[2])
		if err != nil {
			return err
		}
		s.rotRight(a)
	case rotLet:
		a := instFields[6]
		s.rotLetter(a)
	}

	return nil
}

func (s *Scrambler) swapPos(a int, b int) {
	c := s.subject[b]
	s.subject[b] = s.subject[a]
	s.subject[a] = c
}

func (s *Scrambler) swapLetter(a string, b string) {
	aIndex := findByteIndex(s.subject, a[0])
	bIndex := findByteIndex(s.subject, b[0])
	c := s.subject[bIndex]
	s.subject[bIndex] = s.subject[aIndex]
	s.subject[aIndex] = c

}

func (s *Scrambler) rev(a int, b int) {
	revBytes := make([]byte, 0, b-a+1)
	for i := b; i >= a; i-- {
		revBytes = append(revBytes, s.subject[i])
	}

	result := []byte{}

	if a > 0 {
		result = append(result, s.subject[:a]...)
	}

	result = append(result, revBytes...)

	if b < len(s.subject)-1 {
		result = append(result, s.subject[b+1:]...)
	}

	s.subject = result
}

func (s *Scrambler) mov(a int, b int) {
	c := s.subject[a]
	resultA := []byte{}
	resultA = append(resultA, s.subject[:a]...)
	if a < len(s.subject)-1 {
		resultA = append(resultA, s.subject[a+1:]...)
	}

	resultB := []byte{}
	resultB = append(resultB, resultA[:b]...)
	resultB = append(resultB, c)
	if b < len(s.subject)-1 {
		resultB = append(resultB, resultA[b:]...)
	}

	s.subject = resultB
}

func (s *Scrambler) rotLeft(a int) {
	result := s.subject
	for i := 0; i < a; i++ {
		c := result[0]
		result = append(result[1:], c)
	}
	s.subject = result
}

func (s *Scrambler) rotRight(a int) {
	result := s.subject
	for i := 0; i < a; i++ {
		c := result[len(result)-1]
		result = append([]byte{c}, result[:len(result)-1]...)
	}
	s.subject = result
}

func (s *Scrambler) rotLetter(a string) {
	rots := findByteIndex(s.subject, a[0])
	if rots >= 4 {
		rots++
	}
	rots++

	s.rotRight(rots)
}

func getType(instFields []string) (InstType, error) {

	switch instFields[0] {
	case "swap":
		if instFields[1] == "position" {
			return swapPos, nil
		}
		if instFields[1] == "letter" {
			return swapLet, nil
		}
	case "reverse":
		return rev, nil

	case "rotate":
		if instFields[1] == "left" {
			return rotLeft, nil
		}
		if instFields[1] == "right" {
			return rotRight, nil
		}
		if instFields[1] == "based" {
			return rotLet, nil
		}
	case "move":
		return mov, nil
	}

	return invalid, errors.New("invalid instruction")
}
func findByteIndex(bytes []byte, b byte) int {
	for i, s := range bytes {
		if s == b {
			return i
		}
	}
	return -1
}
