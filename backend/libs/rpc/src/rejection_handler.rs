use ::csrf::CSRFCheckFailed;
use ::std::convert::Infallible;
use ::warp::http::StatusCode;
use ::warp::{reject, reply};
use ::warp::{Rejection, Reply};

use crate::entities::Status;

pub async fn handle_rejection(
  err: Rejection,
) -> Result<impl Reply, Infallible> {
  let status;
  if err.is_not_found() {
    status = Status::new(StatusCode::NOT_FOUND, "Not Found");
  } else if let Some(s) = err.find::<Status>() {
    status = s.clone();
  } else if let Some(e) = err.find::<reject::InvalidHeader>() {
    status = Status::new(
      StatusCode::EXPECTATION_FAILED,
      &format!("Invalid Header: {}", e.name()),
    );
  } else if let Some(_) = err.find::<reject::InvalidQuery>() {
    status = Status::new(StatusCode::EXPECTATION_FAILED, "Invalid Query");
  } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
    status = Status::new(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed");
  } else if let Some(_) = err.find::<reject::LengthRequired>() {
    status = Status::new(StatusCode::LENGTH_REQUIRED, "Length Required");
  } else if let Some(e) = err.find::<reject::MissingCookie>() {
    status = Status::new(
      StatusCode::EXPECTATION_FAILED,
      &format!("Missing Cookie: {}", e.name()),
    );
  } else if let Some(e) = err.find::<reject::MissingHeader>() {
    status = Status::new(
      StatusCode::EXPECTATION_FAILED,
      &format!("Missing header: {}", e.name()),
    );
  } else if let Some(_) = err.find::<reject::PayloadTooLarge>() {
    status = Status::new(StatusCode::PAYLOAD_TOO_LARGE, "Payload is too large");
  } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
    status = Status::new(
      StatusCode::UNSUPPORTED_MEDIA_TYPE,
      "The specified media type is not supported.",
    );
  } else if let Some(rej) = err.find::<CSRFCheckFailed>() {
    status = Status::new(
      StatusCode::FORBIDDEN,
      &format!("CSRF Token Mismatch: {}", rej),
    );
  } else {
    status = Status::new(
      StatusCode::SERVICE_UNAVAILABLE,
      &format!("Unhandled Rejection: {:?}", err),
    );
  }
  return Ok(reply::with_status(
    reply::json(&status),
    StatusCode::from_u16(status.code as u16)
      .unwrap_or(StatusCode::SERVICE_UNAVAILABLE),
  ));
}
