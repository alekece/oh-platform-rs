use rocket::{catch, Request};

use crate::{Error, Response};

#[catch(400)]
pub fn bad_request() -> Response<()> {
    Response::Failure(Error::BadRequest(
        "Request is malformed: expected valid JSON".to_string(),
    ))
}

#[catch(404)]
pub fn not_found(request: &Request) -> Response<()> {
    Response::Failure(Error::UnknownRoute(request.uri().to_string()))
}

#[catch(422)]
pub fn unprocessable_entity() -> Response<()> {
    Response::Failure(Error::InvalidData(
        "JSON is well-formed but contains semantic errors".to_string(),
    ))
}
