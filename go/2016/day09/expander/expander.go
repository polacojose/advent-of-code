package expander

import (
	"fmt"
	"strconv"
)

type ReadState int

const (
	EBICopy ReadState = iota
	EBIToken
	EBIExpand
)

type Expander struct {
	source     []byte
	expandBuff []byte
	strPos     int
	state      ReadState
	recursive  bool
}

type token struct {
	length int
	iter   int
}

func NewExpander(source []byte, recursive bool) Expander {
	return Expander{
		source:    source,
		recursive: recursive,
	}
}

func (e *Expander) Expand(buff *[]byte) (int, error) {

	var (
		t              = token{}
		buffWriteCount = 0
	)

	for e.strPos < len(e.source) {

		switch e.state {
		case EBICopy:

			if buffWriteCount >= len(*buff) {
				e.source = e.source[e.strPos:]
				e.strPos = 0
				return buffWriteCount, nil
			}

			e.ebiCopy(&buffWriteCount, buff)

		case EBIToken:

			tok, err := e.ebiToken(&buffWriteCount, buff)
			if err != nil {
				fmt.Println(err)
				return 0, err
			}
			t = tok

		case EBIExpand:
			e.ebiExpand(t, &buffWriteCount, buff)
		}
	}

	return buffWriteCount, nil
}

func (e *Expander) ebiCopy(buffWriteCount *int, buff *[]byte) {
	endCopy := len(e.source)
	for i := e.strPos; i < len(e.source); i++ {
		if e.source[i] == '(' {
			endCopy = i
			break
		}
	}

	cpy := e.source[e.strPos:endCopy]

	for i := 0; i < len(cpy); i++ {
		if *buffWriteCount >= len(*buff) {
			return
		}
		(*buff)[*buffWriteCount] = cpy[i]
		*buffWriteCount++
		e.strPos++
	}

	e.state = EBIToken
	e.strPos++
}

func (e *Expander) ebiToken(buffWriteCount *int, buff *[]byte) (token, error) {

	endToken := 0
	for i := e.strPos; i < len(e.source); i++ {
		if e.source[i] == ')' {
			endToken = i
			break
		}
	}
	tokenString := string(e.source[e.strPos:endToken])

	div := 0
	for i := 0; i < len(tokenString); i++ {
		if tokenString[i] == 'x' {
			div = i
		}
	}

	t := token{}
	a, err := strconv.Atoi(tokenString[:div])
	if err != nil {
		return t, err
	}
	t.length = a

	b, err := strconv.Atoi(tokenString[div+1:])
	if err != nil {
		return t, err
	}
	t.iter = b

	e.state = EBIExpand
	e.strPos += len(tokenString) + 1

	return t, nil
}

func (e *Expander) ebiExpand(t token, buffWriteCount *int, buff *[]byte) {
	e.expandBuff = e.expandBuff[:0]
	for j := 0; j < t.iter; j++ {
		e.expandBuff = append(e.expandBuff, e.source[e.strPos:e.strPos+t.length]...)
	}

	if e.recursive {
		e.source = append(e.source[:0], append(e.expandBuff, e.source[e.strPos+t.length:]...)...)
		e.strPos = 0
	} else {
		for j := 0; j < len(e.expandBuff); j++ {
			(*buff)[*buffWriteCount] = e.expandBuff[j]
			(*buffWriteCount)++
		}
	}

	e.state = EBICopy
}
