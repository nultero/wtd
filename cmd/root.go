package cmd

import (
	"fmt"
	"wtf/cmd/search"
	"wtf/cmd/sorts"

	"github.com/nultero/tics"
	"github.com/spf13/cobra"
)

var Debug bool = false
var Depth int = 3
var NoStrip bool = false
var PrintAnyway bool = false
var Reverse bool = false
var Verbose bool = false
var SearchStr string

// TODO write up a cachefile implementation so the file search takes less time

var wtf string = fmt.Sprintf(
	"%vhat %vhe %vix",
	tics.Make("w").Blue().String(),
	tics.Make("t").Blue().String(),
	tics.Make("f").Blue().String(),
)

var todo string = tics.Make("TODO").Blue().String()
var osOnTheEnd = tics.Make("O").Blue().String()

var rootCmd = &cobra.Command{
	Use: "wtf",
	Long: wtf + ": searches current dir files \nfor '" +
		todo + "'s, and prioritizes them on the \nnumber of " +
		osOnTheEnd + "s on the end --\n\n" +
		"e.g., 'TODOOOO' is a higher-priority todo than 'TODO'",

	// urgency, context, file and line no#

	Run: func(cmd *cobra.Command, args []string) {

		lines := search.Default(&Verbose, &NoStrip, &Debug, &PrintAnyway)

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
	rootCmd.Flags().BoolVarP(&Debug, "debug", "g", false, "strips split str, useful only if SortLines throws an error (extremely rare)")
	rootCmd.Flags().BoolVarP(&NoStrip, "nostrip", "n", false, "prevents whitespace / tabs from being stripped when combing lines (for verbose flag only)")
	rootCmd.Flags().BoolVarP(&PrintAnyway, "print", "p", false, "prints items as the channel iterates over a directory -- for whatever reason, some repos don't play nice")
	rootCmd.Flags().BoolVarP(&Reverse, "reverse", "r", false, "least-priority results are listed first")
	rootCmd.Flags().BoolVarP(&Verbose, "verbose", "v", false, "prints line contents of given TODO (but not multiline comments)")
	rootCmd.Flags().StringVarP(&SearchStr, "searchstr", "s", "TODO", "searches for this string instead of some other, but does not compute priority")
	// -d for depth?
}
