package search

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"wtf/cmd/ignores"

	"github.com/nultero/tics"
)

// The default search behavior for scanning a directory's
// scattered TODOs.
func Default() {
	here, err := os.Getwd()
	if err != nil {
		tics.ThrowSys(Default, err)
	}

	im := ignores.BasicIgMap()
	c := make(chan string)
	go search(here, c, &im)

	lines := []string{}
	for s := range c {
		lines = append(lines, s)
		fmt.Println(s)
	}

	fmt.Println(lines)

}

func search(dir string, c chan string, ignoreMap *ignores.Ignore) {
	files, err := ioutil.ReadDir(dir)
	if err != nil {
		tics.ThrowSys(search, err)
	}

	for _, f := range files {
		if ignoreMap.NotExcluded(f.Name()) {

			if f.IsDir() {
				subchan := make(chan string)
				go search(dir+"/"+f.Name(), subchan, ignoreMap)
				for s := range subchan {
					c <- s
				}

			} else {
				subchan := make(chan string)
				go readFile(dir, f.Name(), subchan)
				for s := range subchan {
					c <- s
				}
			}
		}
	}

	defer close(c)
}

func readFile(dir, fname string, c chan string) {
	b, err := os.ReadFile(dir + "/" + fname)
	if err != nil {
		tics.ThrowSys(readFile, err)
	}

	cont := string(b)
	lines := strings.Split(cont, "\n")

	for i, ln := range lines {
		if strings.Contains(ln, "TODO") {
			s := "<" + fname + "> contains TODO at line " + fmt.Sprint(i+1)
			c <- s
		}
	}

	close(c)
}
