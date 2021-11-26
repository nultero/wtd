package cmd

import (
	"fmt"
	"wtf/cmd/search"

	"github.com/nultero/tics"
	"github.com/spf13/cobra"
)

var Depth int = 3
var Reverse bool = false

// var Threads int = 10 // goroutines to spin up to search dir

// TODO write up a cachefile implementation so the file search takes less time

var wtf string = fmt.Sprintf(
	"%vhat %vhe %vix",
	tics.Blue("w"),
	tics.Blue("t"),
	tics.Blue("f"),
)

var todo string = tics.Blue("TODO")
var osOnTheEnd = tics.Blue("O")

var rootCmd = &cobra.Command{
	Use: "wtf",
	Long: wtf + ": searches current dir files \nfor '" +
		todo + "'s, and prioritizes them on the \nnumber of " +
		osOnTheEnd + "s on the end --\n\n" +
		"e.g., 'TODOOOO' is a higher-priority todo than 'TODO'",

	// urgency, context, file and line no#

	Run: func(cmd *cobra.Command, args []string) {
		search.Default()
	},
}

func Execute() {
	cobra.CheckErr(rootCmd.Execute())
}

func init() {
	rootCmd.Flags().BoolVarP(&Reverse, "reverse", "r", false, "least-priority results are listed first")
	// -d for depth?
	// -s for search str that isn't 'TODO'
}
