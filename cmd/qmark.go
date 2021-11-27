package cmd

import (
	"fmt"

	"github.com/nultero/tics"
)

var qLines = []string{
	"    ____",
	"  /  __ \\",
	"  | |  | |",
	"  ‾‾‾  / |  > no 'TODOS' found",
	"      / /",
	"     | /",
	"     ‾‾  ",
	"     ◯  ",
}

// The print for no 'TODO' comments found, or else no matching arguments.
func printQmark() {
	for _, ln := range qLines {
		fmt.Println(tics.Blue(ln))
	}
}
