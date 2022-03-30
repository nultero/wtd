package main

import (
	"os"
	"runtime/pprof"
	"wtf/out"
)

func main() {

	f, err := os.Create("cpuprof")
	if err != nil {
		panic(err)
	}

	err = pprof.StartCPUProfile(f)
	if err != nil {
		panic(err)
	}

	defer pprof.StopCPUProfile()

	flags, filesToSearch := parseArgs(os.Args[1:])
	if flags.help {
		out.PrintHelp()
		return
	}

	fts, ok := filesToSearch.([]string)

	if !ok || len(fts) == 0 { // do normal search
		searchCwd(flags)

	} else { // do specific search if there are file args
		searchCwd(flags, fts...)
	}
}

func parseArgs(args []string) (flags, any) {
	flags := defaultFlags()
	filesToSearch := []string{}
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
			filesToSearch = append(filesToSearch, arg)
		}
	}

	if len(filesToSearch) == 0 {
		return flags, struct{}{}
	} else {
		return flags, filesToSearch
	}
}
