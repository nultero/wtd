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

const LineSplitStr = "¿¿" // literally no one will use two of these in a comment that contains TODO
// 100% safe to use a str delimiter

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
			"%vpriority: %v",
			ln.Line,
			ln.Priority,
		))
	}

	return lines
}
