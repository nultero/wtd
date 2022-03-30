package main

type flags struct {
	groupby, help, noStrip, reverse bool
	verbose                         uint8
}

func defaultFlags() flags {
	return flags{
		groupby: false,
		help:    false,
		noStrip: false,
		reverse: false,
		verbose: 0,
	}
}
