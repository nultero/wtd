package main

import (
	"fmt"
	"os"
)

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
	searchDir(cwd, matches)
}

func searchDir(dir string, m map[string][]match) {
	curDir, err := os.ReadDir(dir)
	if err != nil {
		panic(err)
	}
	for _, f := range curDir {
		if !f.IsDir() {
			searchFile(f.Name(), m)
		}
	}
}

func searchFile(file string, m map[string][]match) {
	bytes, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}

	str := string(bytes)
	ln := 0
	for _, char := range str {
		if char == '\n' {
			ln++
		}
	} // clean up search funcs

	// fmt.Println(ln, file)
}
