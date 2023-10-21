use ::kvs::SymbolKVSBuilder;

#[allow(non_upper_case_globals)]
pub const CurrentSyncProgressStoreBuilder: SymbolKVSBuilder<i64> =
  SymbolKVSBuilder::new("kline_sync:current");

#[allow(non_upper_case_globals)]
pub const NumObjectsToFetchStoreBuilder: SymbolKVSBuilder<i64> =
  SymbolKVSBuilder::new("kline_sync:num");
