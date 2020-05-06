package history

// HistoricalPriceDataDownloader represents an interface to download
// 1-min split historical chart data.
type HistoricalPriceDataDownloader interface {
	run(pair string)
}
