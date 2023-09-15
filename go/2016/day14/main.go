package main

import (
	"crypto/md5"
	"fmt"
	"io"
)

type PotentialKey struct {
	hash   string
	triple byte
	index  int
}

var hashes map[int]string

const salt = "ihaygndm"

func init() {
	hashes = map[int]string{}
}

func main() {

	hashes = map[int]string{}
	fmt.Println("Basic")
	basic()

	hashes = map[int]string{}
	fmt.Println("Stretched")
	stretched()
}

func basic() {
	keys := []string{}
	indexes := []int{}
	for i := 0; len(indexes) < 64; i++ {

		hash := getHash(i)

		triple, ok := hasTriple(hash)
		if !ok {
			continue
		}

		if isKey(i, triple, getHash) {
			indexes = append(indexes, i)
			keys = append(keys, hash)
			continue
		}

	}

	fmt.Println("64th Index:", indexes[len(indexes)-1])
}

func stretched() {
	keys := []string{}
	indexes := []int{}
	for i := 0; len(indexes) < 64; i++ {

		hash := getStretchedHash(i)

		triple, ok := hasTriple(hash)
		if !ok {
			continue
		}

		if isKey(i, triple, getStretchedHash) {
			indexes = append(indexes, i)
			keys = append(keys, hash)
			continue
		}

	}

	fmt.Println("64th Index:", indexes[len(indexes)-1])
}

func getHash(i int) string {
	var hash string
	if h, ok := hashes[i]; ok {
		hash = h
	} else {
		h := md5.New()
		io.WriteString(h, fmt.Sprintf("%s%d", salt, i))
		s := []byte{}
		s = h.Sum(s)
		hash = fmt.Sprintf("%x", string(s))
		hashes[i] = hash
	}
	return hash
}

func getStretchedHash(i int) string {
	var hash string
	if h, ok := hashes[i]; ok {
		hash = h
	} else {
		input := fmt.Sprintf("%s%d", salt, i)
		for j := 0; j < 2017; j++ {
			h := md5.New()
			io.WriteString(h, input)
			s := []byte{}
			s = h.Sum(s)
			input = fmt.Sprintf("%x", string(s))
		}
		hash = input
		hashes[i] = hash
	}
	return hash
}

func hasTriple(hash string) (triple byte, ok bool) {
	triple = byte(0)
	ok = false
	for i := 2; i < len(hash); i++ {
		a := hash[i]
		b := hash[i-1]
		c := hash[i-2]
		if a == b && b == c {
			triple = a
			ok = true
			return triple, ok
		}
	}
	return triple, ok
}

func isKey(i int, triple byte, hashFunction func(int) string) bool {
	for j := i + 1; j < i+1000; j++ {
		h := hashFunction(j)
		for k := 4; k < len(h); k++ {
			a := h[k-4]
			b := h[k-3]
			c := h[k-2]
			d := h[k-1]
			e := h[k]

			if a == triple && a == b && b == c && c == d && d == e {
				return true
			}
		}
	}

	return false
}
