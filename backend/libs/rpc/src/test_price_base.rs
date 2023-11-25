
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum TestPriceBase {
  Close,
  High,
  HighLowMid,
  Low,
  Open,
  OpenCloseMid,
}
