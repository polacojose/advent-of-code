package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type IP7 struct {
	sn []string
	hn []string
}

type IpScanState int

const (
	ScanningSupernet IpScanState = 0
	ScanningHypernet IpScanState = 1
)

func parseIP7(line string) IP7 {

	nonHypernets := []string{}
	hypernets := []string{}

	buff := []rune{}
	state := ScanningSupernet
	for _, r := range line {
		switch state {
		case ScanningSupernet:
			if r == '[' {
				nonHypernets = append(nonHypernets, string(buff))
				buff = []rune{}
				state = ScanningHypernet
				continue
			}

			buff = append(buff, r)

		case ScanningHypernet:
			if r == ']' {
				hypernets = append(hypernets, string(buff))
				buff = []rune{}
				state = ScanningSupernet
				continue
			}
			buff = append(buff, r)
		}
	}
	nonHypernets = append(nonHypernets, string(buff))

	return IP7{
		sn: nonHypernets,
		hn: hypernets,
	}
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	fileScanner := bufio.NewScanner(f)
	fileScanner.Split(bufio.ScanLines)

	ips := []IP7{}
	for fileScanner.Scan() {
		line := fileScanner.Text()
		addr := parseIP7(line)
		ips = append(ips, addr)
	}

	supportsTLS := 0
	supportsSSL := 0
	for _, ip := range ips {
		if ipSupportTLS(ip) {
			supportsTLS++
		}
		if ipSupportSSL(ip) {
			supportsSSL++
		}
	}

	fmt.Println("Total Support TLS:", supportsTLS)
	fmt.Println("Total Support SSL:", supportsSSL)

}

func ipSupportTLS(ip IP7) bool {
	for _, s := range ip.hn {
		for i := 3; i < len(s); i++ {
			if s[i-3] != s[i-2] && s[i-3] == s[i] && s[i-1] == s[i-2] {
				return false
			}
		}
	}
	for _, s := range ip.sn {
		for i := 3; i < len(s); i++ {
			if s[i-3] != s[i-2] && s[i-3] == s[i] && s[i-1] == s[i-2] {
				return true
			}
		}
	}
	return false
}

func ipSupportSSL(ip IP7) bool {

	babs := []string{}
	for _, s := range ip.sn {
		for i := 2; i < len(s); i++ {
			if s[i-2] != s[i-1] && s[i-2] == s[i] {
				bab := string([]byte{s[i-1], s[i-2], s[i-1]})
				babs = append(babs, bab)
			}
		}
	}

	for _, aba := range babs {
		for _, hn := range ip.hn {
			if strings.Index(hn, aba) >= 0 {
				return true
			}
		}
	}

	return false
}
