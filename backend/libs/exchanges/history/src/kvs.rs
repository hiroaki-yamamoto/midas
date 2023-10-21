use ::kvs::SymbolKVSBuilder;

#[allow(non_upper_case_globals)]
pub const CurrentSyncProgressStore: SymbolKVSBuilder<i64> =
  SymbolKVSBuilder::new("kline_sync:current");
#[allow(non_upper_case_globals)]
pub const NumObjectsToFetchStore: SymbolKVSBuilder<i64> =
  SymbolKVSBuilder::new("kline_sync:num");
