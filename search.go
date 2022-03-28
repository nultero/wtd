package main

import (
	"fmt"
	"os"
)

func searchCwd() {
	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	dir, err := os.ReadDir(cwd)
	if err != nil {
		panic(err)
	}

	for _, f := range dir {
		fmt.Println(f.Name())
	}
}
