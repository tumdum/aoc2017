package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
)

func sum(in []int) int {
	r := 0
	for i := 1; i != len(in); i++ {
		if in[i-1] == in[i] {
			r += in[i]
		}
	}
	if in[0] == in[len(in)-1] {
		r += in[0]
	}
	return r
}

func getByStep(in []int, pos int, step int) int {
	dst := (pos + step) % len(in)
	return in[dst]
}

func sumB(in []int) int {
	step := len(in) / 2
	r := 0
	for i := 0; i != len(in); i++ {
		other := getByStep(in, i, step)
		if in[i] == other {
			r += in[i]
		}
	}
	return r
}

func parse(in string) []int {
	ret := []int{}
	for _, c := range in {
		ret = append(ret, int(c)-'0')
	}
	return ret
}

func solve(in string) int {
	return sum(parse(in))
}

func solveB(in string) int {
	return sumB(parse(in))
}

func main() {
	fmt.Println(solve("1122"))
	fmt.Println(solve("1111"))
	fmt.Println(solve("1234"))
	fmt.Println(solve("91212129"))

	fmt.Println(solveB("1212"))
	fmt.Println(solveB("1122"))
	fmt.Println(solveB("123425"))
	fmt.Println(solveB("123123"))
	fmt.Println(solveB("12131415"))

	var buf bytes.Buffer
	io.Copy(&buf, os.Stdin)
	input := buf.String()
	input = strings.TrimSpace(input)
	fmt.Println("a:", solve(input))
	fmt.Println("b:", solveB(input))
}
