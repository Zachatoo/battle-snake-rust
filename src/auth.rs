use std::env;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key == match env::var("API_KEY") {
                Ok(value) => value,
                Err(_) => "valid_api_key".to_string(),
            }
        }

        match req.query_value::<&str>("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => match key {
                Ok(value) if is_valid(value) => Outcome::Success(ApiKey(value)),
                Ok(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
                Err(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            },
        }
    }
}
