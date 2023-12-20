pub mod interfaces;
mod rest;
mod ws;

pub use ::reqwest;

pub use crate::rest::RestClient;
pub use crate::ws::WebSocket;

#[cfg(test)]
mod test_ws;
