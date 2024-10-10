use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    ServiceUnavailable = 503,
    UnprocessableEntity = 422,
    UnsupportedMediaType = 415,
    MethodNotAllowed = 405,
    TooManyRequests = 429,
    GatewayTimeout = 504,
    UnavailableForLegalReasons = 451,
}
