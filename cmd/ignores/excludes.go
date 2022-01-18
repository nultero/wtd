package ignores

type IgnoreMap map[string]struct{}

func BasicIgMap() IgnoreMap {
	return IgnoreMap{".git": {}}
}

func (i IgnoreMap) NotExcluded(filename string) bool {
	_, ok := i[filename]
	return !ok
}
