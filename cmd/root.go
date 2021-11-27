package cmd

import (
	"fmt"
	"wtf/cmd/search"
	"wtf/cmd/sorts"

	"github.com/nultero/tics"
	"github.com/spf13/cobra"
)

var Depth int = 3
var Reverse bool = false
var Verbose bool = false
var SearchStr string

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

		lines := search.Default(&Verbose)

		if len(lines) == 0 {
			printQmark()

		} else {
			lines = sorts.SortLines(lines, &Reverse)
			printLines(lines)
		}
	},
}

func Execute() {
	cobra.CheckErr(rootCmd.Execute())
}

func init() {
	rootCmd.Flags().BoolVarP(&Reverse, "reverse", "r", false, "least-priority results are listed first")
	rootCmd.Flags().BoolVarP(&Verbose, "verbose", "v", false, "prints line contents of given TODO (but not multiline comments)")
	rootCmd.Flags().StringVarP(&SearchStr, "searchstring", "s", "TODO", "searches for this string instead of some other, but does not compute priority")
	// -d for depth?
}
