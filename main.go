package main

import (
	"fmt"
	"os"
	"wtf/out"
)

func main() {
	flags, filesToSearch := parseArgs(os.Args[1:])
	if flags.help {
		out.PrintHelp()
		return
	}

	fts, ok := filesToSearch.([]string)

	if !ok || len(fts) == 0 {
		searchCwd()

	} else {
		fmt.Println(fts)
	}
}

func parseArgs(args []string) (flags, any) {
	flags := defaultFlags()
	searches := []string{}
	const u3 uint8 = 3

	for _, arg := range args {
		if arg[0] == '-' {
			for _, char := range arg[1:] {
				switch char {
				case 'h':
					flags.help = true
				case 'n':
					flags.noStrip = true
				case 'r':
					flags.reverse = true
				case 'v':
					if flags.verbose < u3 {
						flags.verbose++
					}
				}
			}

		} else {
			searches = append(searches, arg)
		}
	}

	if len(searches) == 0 {
		return flags, struct{}{}
	} else {
		return flags, searches
	}
}
