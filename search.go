package main

import (
	"fmt"
	"os"
	"strings"
)

const keywd = "TODO"

// TODOOOOOO respect gitignores

func searchCwd(f flags, files ...string) {

	if len(files) != 0 {
		fmt.Println("not zero args")
		return
	}

	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	matches := map[string][]match{}
	searchDir(cwd, matches, f)

	// TODO impl all the prints
}

func searchDir(dir string, m map[string][]match, fg flags) {
	curDir, err := os.ReadDir(dir)
	if err != nil {
		panic(err)
	}
	for _, f := range curDir {
		if !f.IsDir() {
			searchFile(f.Name(), m, fg.verbose, fg.noStrip)
		}
	}
}

func searchFile(file string, m map[string][]match, verbosity uint8, nostrip bool) {
	bytes, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}

	str := string(bytes)
	spl := strings.Split(str, "\n")
	mt := []match{}

	for _, ln := range spl {
		if strings.Contains(ln, keywd) {
			// o := countOs(ln)
		}
	}

	fmt.Printf("%v", mt)
	// fmt.Println(file, ln, count)
}

func countOs(s string) int {

	// tried to optimize this a bit, by slicing, counting,
	// all a lot dumber than naive splitting
	// lots of off-by-one errors

	// TODOOOOOOOOOOOOOOOOOOOOooooo finish counting Os
	// f**k it, I'll come back to this later fresh

	return 5
}
