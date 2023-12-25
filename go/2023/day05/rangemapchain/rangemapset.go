package rangemapchain

type rangeMapSet struct {
	name      string
	mapRanges []mapRange
}

func (r *rangeMapSet) destFromSource(s int) int {
	for _, mr := range r.mapRanges {
		if mr.source <= s && s < mr.source+mr.length {
			return mr.destination + s - mr.source
		}
		if mr.source > s {
			return s
		}
	}
	return s
}
