use rocket::http::Status;
use rocket::response::{Responder, Response as RocketResponse, Result as ResponseResult};
use rocket::serde::json::Json;
use rocket::Request;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner};
use schemars::JsonSchema;
use serde::Serialize;

use crate::error::Error;

pub enum Response<T> {
    Success { data: T, status: Status },
    Failure(Error),
}

impl<'r, T: Serialize> Responder<'r, 'static> for Response<T> {
    fn respond_to(self, request: &'r Request<'_>) -> ResponseResult<'static> {
        let (status, mut response) = match self {
            Response::Success { data, status } => (
                status,
                RocketResponse::build_from(Json(DataEnvelop { data }).respond_to(request)?),
            ),
            Response::Failure(e) => (
                e.get_http_status(),
                RocketResponse::build_from(Json::<OpaqueError>(e.into()).respond_to(request)?),
            ),
        };

        response.status(status).ok()
    }
}

impl<T> OpenApiResponderInner for Response<T>
where
    T: JsonSchema,
{
    fn responses(generator: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut responses = Responses::default();

        rocket_okapi::util::add_schema_response(
            &mut responses,
            200,
            "application/json",
            generator.json_schema::<DataEnvelop<T>>(),
        )?;

        rocket_okapi::util::add_default_response_schema(
            &mut responses,
            "application/json",
            generator.json_schema::<OpaqueError>(),
        );

        Ok(responses)
    }
}

pub trait IntoResponse<T> {
    fn into_response(self, status: Status) -> Response<T>;
}

impl<T, E> IntoResponse<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn into_response(self, status: Status) -> Response<T> {
        match self {
            Ok(data) => Response::Success { data, status },
            Err(e) => Response::Failure(e.into()),
        }
    }
}

#[derive(JsonSchema, Clone, Serialize)]
struct DataEnvelop<T> {
    data: T,
}

#[derive(JsonSchema, Serialize)]
struct OpaqueError {
    error: String,
    code: u16,
}

impl From<Error> for OpaqueError {
    fn from(e: Error) -> Self {
        Self {
            error: e.to_string(),
            code: e.get_http_status().code,
        }
    }
}
