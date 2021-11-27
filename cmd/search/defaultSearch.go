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

// The default search behavior for scanning a directory's
// scattered TODOOOOOOs.
func Default(verbose *bool) []string {
	here, err := os.Getwd()
	if err != nil {
		tics.ThrowSys(Default, err)
	}

	im := ignores.BasicIgMap()
	c := make(chan string)
	go search(here, c, &im, verbose)

	lines := []string{}
	for s := range c {
		lines = append(lines, s)
	}

	return lines
}

func search(dir string, c chan string, ignoreMap *ignores.Ignore, verbose *bool) {
	files, err := ioutil.ReadDir(dir)
	if err != nil {
		tics.ThrowSys(search, err)
	}

	for _, f := range files {
		if ignoreMap.NotExcluded(f.Name()) {

			if f.IsDir() {
				subchan := make(chan string)
				go search(dir+"/"+f.Name(), subchan, ignoreMap, verbose)
				for s := range subchan {
					c <- s
				}

			} else {
				subchan := make(chan string)
				go readFile(dir, f.Name(), subchan, verbose)
				for s := range subchan {
					c <- s
				}
			}
		}
	}

	defer close(c)
}

func readFile(dir, fname string, c chan string, verbose *bool) {
	b, err := os.ReadFile(dir + "/" + fname)
	if err != nil {
		tics.ThrowSys(readFile, err)
	}

	cont := string(b)
	lines := strings.Split(cont, "\n")

	for i, ln := range lines {
		if strings.Contains(ln, "TODO") {

			s := ""
			priorityOfOs := countOs(ln)

			if *verbose {
				s = fmt.Sprintf(
					"<%v> line: %v, `%v`, %v%v",
					tics.DarkBlue(fname),
					i+1,                //ln #
					ln,                 // verbose
					sorts.LineSplitStr, // for processing later
					priorityOfOs,
				)

			} else {
				s = fmt.Sprintf(
					"<%v> line: %v, %v%v",
					tics.DarkBlue(fname),
					i+1, //ln #
					sorts.LineSplitStr,
					priorityOfOs,
				)
			}

			c <- s
		}
	}

	close(c)
}

// Counts the 'O's on the end of a TODO to determine its priority rating.
func countOs(line string) int {

	spl := strings.Split(line, "TOD")

	s := spl[1]
	numOs := 0 // default 'O' priority would be 1 I suppose
	r := s[0]

	for len(s) > 0 && r == 'O' { // just advances a char until char is not 'O'
		numOs++
		s = s[1:]
		if len(s) != 0 {
			r = s[0]
		}
	}

	return numOs
}
