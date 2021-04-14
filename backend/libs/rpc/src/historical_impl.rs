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
