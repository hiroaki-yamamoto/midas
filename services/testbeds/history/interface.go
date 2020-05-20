package history

import "sync"

// HistoricalPriceDataDownloader represents an interface to download
// 1-min split historical chart data.
type HistoricalPriceDataDownloader interface {
	Run(pair string) error
	GetWaitGroup() *sync.WaitGroup
}
