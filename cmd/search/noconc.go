package search

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"wtf/cmd/ignores"
	"wtf/cmd/sorts"

	"github.com/nultero/tics"
)

func NoConc(verbose, nostrip, debug, printAnyway *bool) []string {
	here, err := os.Getwd()
	if err != nil {
		tics.ThrowSys(Default, err)
	}

	im := ignores.BasicIgMap()

	lines := []string{}
	baseSearch(here, &lines, &im, verbose, nostrip, debug)

	for l := range lines {
		if *printAnyway {
			fmt.Println(l)
		}
	}

	return lines
}

func baseSearch(dir string, lines *[]string, ignoreMap *ignores.IgnoreMap, verbose, nostrip, debug *bool) {
	files, err := ioutil.ReadDir(dir)
	if err != nil {
		tics.ThrowSys(search, err)
	}

	for _, f := range files {

		if ignoreMap.NotExcluded(f.Name()) {

			if f.IsDir() {
				baseSearch(dir+"/"+f.Name(), lines, ignoreMap, verbose, nostrip, debug)

			} else {
				readFileNoConc(dir, f.Name(), lines, verbose, nostrip, debug)
			}
		}
	}
}

func readFileNoConc(dir, fname string, lines *[]string, verbose, nostrip, debug *bool) {
	b, err := os.ReadFile(dir + "/" + fname)
	if err != nil {
		tics.ThrowSys(readFile, err)
	}

	cont := string(b)
	fileLines := strings.Split(cont, "\n")

	for i, ln := range fileLines {
		if strings.Contains(ln, "TODO") {

			s := fmt.Sprintf(
				"<%v> line: %v, ",
				tics.Make(fname).DarkBlue().String(),
				i+1, // ln #
			)

			priorityOfOs := countOs(ln)

			if *verbose {
				if *nostrip {
					s += fmt.Sprintf("`%v`, ", ln)

				} else {
					ln = strings.TrimLeft(ln, " \t")
					s += fmt.Sprintf("`%v`, ", ln)
				}
			}

			if *debug {
				s = strings.ReplaceAll(s, sorts.LineSplitStr, "")
			}

			s += fmt.Sprintf("%v%v", sorts.LineSplitStr, priorityOfOs)
			*lines = append(*lines, s)
		}
	}
}
