use ::kvs::incr_kvs;

incr_kvs!(
  pub,
  CurrentSyncProgressStore,
  i64,
  "{}:{}:kline_sync:current"
);
incr_kvs!(pub, NumObjectsToFetchStore, i64, "{}:{}:kline_sync:num");
