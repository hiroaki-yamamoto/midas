mod attempt;
mod empty;
mod execution;
mod initialize;
mod object;
mod parse;
mod status;
mod websocket;

pub use attempt::MaximumAttemptExceeded;
pub use empty::EmptyError;
pub use execution::ExecutionFailed;
pub use initialize::InitError;
pub use object::ObjectNotFound;
pub use parse::ParseError;
pub use status::StatusFailure;
pub use websocket::WebsocketError;
