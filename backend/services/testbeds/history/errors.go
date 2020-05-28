package history

import "fmt"

// NoSuchPair indicates an error that denotes there's no such a trading pair.
type NoSuchPair struct {
	Pair string
}

// Error implements error interface.
func (me *NoSuchPair) Error() string {
	return fmt.Sprintf("No such pair: %s", me.Pair)
}
