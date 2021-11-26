package ignores

type Ignore struct {
	Map map[string]struct{}
}

func BasicMap() Ignore {
	return Ignore{
		Map: map[string]struct{}{
			".git": {},
		},
	}
}

func (i *Ignore) NotExcluded(filename string) bool {
	_, ok := i.Map[filename]
	return !ok
}
