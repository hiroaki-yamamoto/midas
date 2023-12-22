use ::kvs::SymbolKVSBuilder;

pub const CUR_SYNC_PROG_KVS_BUILDER: SymbolKVSBuilder<i64> =
  SymbolKVSBuilder::new("kline_sync:current");

pub const NUM_TO_FETCH_KVS_BUILDER: SymbolKVSBuilder<i64> =
  SymbolKVSBuilder::new("kline_sync:num");
