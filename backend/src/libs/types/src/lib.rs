use ::std::result::Result as StdResult;
use ::tonic::Status;

pub type Result<T> = StdResult<T, Status>;
