package expandcounter

import (
	"strconv"
	"strings"
)

type ExpandCounterState int

const (
	RawCopy ExpandCounterState = iota
	TokenDecode
)

type ExpandCounter struct {
	referenceString string
	state           ExpandCounterState
	pos             int
	recursive       bool
}

func NewExpandCounter(referenceString string, recursive bool) *ExpandCounter {
	return &ExpandCounter{
		state:           RawCopy,
		referenceString: referenceString,
		recursive:       recursive,
	}
}

func (e *ExpandCounter) Count() uint {
	var total uint = 0
	for e.pos < len(e.referenceString) {
		switch e.state {
		case RawCopy:
			total += e.RawCopy()
		case TokenDecode:
			total += e.TokenDecode()
		}
	}
	return total
}

func (e *ExpandCounter) RawCopy() uint {

	loc := strings.Index(e.referenceString[e.pos:], "(")
	if loc == -1 {
		l := len(e.referenceString[e.pos:])
		e.pos += len(e.referenceString[e.pos:])
		return uint(l)
	}

	var total = loc
	e.state = TokenDecode
	e.pos += loc

	return uint(total)
}

func (e *ExpandCounter) TokenDecode() uint {

	loc := strings.Index(e.referenceString[e.pos:], ")")
	expandParts := strings.Split(e.referenceString[e.pos+1:e.pos+loc], "x")
	chars, _ := strconv.Atoi(expandParts[0])
	orgChars := chars
	iter, _ := strconv.Atoi(expandParts[1])

	if e.recursive {
		subSearchString := e.referenceString[e.pos+loc+1 : e.pos+loc+1+chars]
		subLoc := strings.Index(subSearchString, "(")
		if subLoc != -1 {
			subCounter := NewExpandCounter(subSearchString, true)
			chars = int(subCounter.Count())
		}
	}

	e.pos += loc + 1 + orgChars
	e.state = RawCopy
	return uint(chars * iter)
}
