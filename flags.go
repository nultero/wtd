package main

type flags struct {
	help, noStrip, reverse bool
	verbose                uint8
}

func defaultFlags() flags {
	return flags{
		help:    false,
		noStrip: false,
		reverse: false,
		verbose: 0,
	}
}
