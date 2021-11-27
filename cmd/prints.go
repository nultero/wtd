package cmd

import "fmt"

func printLines(lines []string) {

	for _, ln := range lines {
		fmt.Println(ln)
	}

}
