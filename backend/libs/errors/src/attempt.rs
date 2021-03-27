use ::err_derive::Error;

#[derive(Debug, Clone, Default, Error)]
#[error(display = "Maximum retrieving count exceeded.")]
pub struct MaximumAttemptExceeded;
