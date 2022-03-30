package main

import "testing"

func Test_CountOs(t *testing.T) {

	testStrs := []string{
		"TODo", // 0
		" TODO",
		" TODOO0OO)",      // 2
		"1 TOD TODOOOooo", // 3
		"TODOOOO",
		"TODOOOOO\n",    // 5
		"    TODOOOOOO", // 6
	}

	for i, s := range testStrs {
		c := countOs(s)
		if c != i {
			t.Errorf("wanted %d at index %d, got %d, str:%s", i, i, c, testStrs[i])
		}
	}
}
