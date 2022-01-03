package sorts

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/nultero/tics"
)

type linkedLine struct {
	Line     string
	Priority int
}

const LineSplitStr = "¿¿" // literally no one will use two of these side-by-side in a comment that contains TODO
// 100% safe to use a str delimiter
// EXCEPT ON THIS FILE, because self-referencing this above line causes an err
// I've added a debug flag so that wtf can be run on its own dir and not have problems

func SortLines(lines []string, reversed *bool) []string {

	sl := []linkedLine{}
	for _, ln := range lines {

		split := strings.Split(ln, LineSplitStr)
		p, err := strconv.Atoi(split[1])
		if err != nil {
			tics.ThrowSys(SortLines, err)
		}

		sl = append(sl, linkedLine{
			Line:     split[0],
			Priority: p,
		})
	}

	if *reversed {
		sort.Slice(sl, func(i, j int) bool {
			return sl[i].Priority < sl[j].Priority
		})

	} else {
		sort.Slice(sl, func(i, j int) bool {
			return sl[i].Priority > sl[j].Priority
		})
	}

	lines = []string{}
	for _, ln := range sl {
		lines = append(lines, fmt.Sprintf(
			"%v%v: %v",
			ln.Line,
			tics.Make("priority").Magenta().String(),
			ln.Priority,
		))
	}

	return lines
}
