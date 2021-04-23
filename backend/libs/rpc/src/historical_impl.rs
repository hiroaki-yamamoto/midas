use ::std::cmp::{Ord, Ordering, PartialOrd};
use ::std::ops::{Add, Sub};

use ::http::status::StatusCode;

use super::entities::Status;
use super::historical::HistChartProg;

impl Sub for &HistChartProg {
  type Output = Result<HistChartProg, Status>;
  fn sub(self, rhs: Self) -> Self::Output {
    if self.symbol != rhs.symbol {
      return Err(Status::new(
        StatusCode::EXPECTATION_FAILED,
        "The symbol must be the same for each other.",
      ));
    }
    return Ok(HistChartProg {
      symbol: self.symbol.clone(),
      num_symbols: self.num_symbols - rhs.num_symbols,
      num_objects: self.num_objects - rhs.num_objects,
      cur_symbol_num: self.cur_symbol_num - rhs.cur_symbol_num,
      cur_object_num: self.cur_object_num - rhs.cur_object_num,
    });
  }
}

impl Sub for HistChartProg {
  type Output = Result<HistChartProg, Status>;
  fn sub(self, rhs: Self) -> Self::Output {
    return &self - &rhs;
  }
}

impl Add for &HistChartProg {
  type Output = Result<HistChartProg, Status>;
  fn add(self, rhs: Self) -> Self::Output {
    if self.symbol != rhs.symbol {
      return Err(Status::new(
        StatusCode::EXPECTATION_FAILED,
        "The symbol must be the same for each other.",
      ));
    }
    return Ok(HistChartProg {
      symbol: self.symbol.clone(),
      num_symbols: self.num_symbols + rhs.num_symbols,
      num_objects: self.num_objects + rhs.num_objects,
      cur_symbol_num: self.cur_symbol_num + rhs.cur_symbol_num,
      cur_object_num: self.cur_object_num + rhs.cur_object_num,
    });
  }
}

impl Add for HistChartProg {
  type Output = Result<HistChartProg, Status>;
  fn add(self, rhs: Self) -> Self::Output {
    return &self + &rhs;
  }
}

impl Ord for HistChartProg {
  fn cmp(&self, other: &Self) -> Ordering {
    let symbol_cmp = self.cur_symbol_num.cmp(&other.cur_symbol_num);
    if symbol_cmp != Ordering::Equal {
      return symbol_cmp;
    }
    return self.cur_object_num.cmp(&other.cur_object_num);
  }
}

impl PartialOrd for HistChartProg {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}
