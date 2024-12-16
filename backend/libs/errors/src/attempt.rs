use ::thiserror::Error;

#[derive(Debug, Clone, Default, Error)]
#[error("Maximum retrieving count exceeded.")]
pub struct MaximumAttemptExceeded;
