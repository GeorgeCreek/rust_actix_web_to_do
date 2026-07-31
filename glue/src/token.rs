use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, 
    encode, 
    Algorithm, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation
};
use std::time::{SystemTime, UNIX_EPOCH};


fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}


fn access_ttl_secs() -> u64 {
    std::env::var("ACCESS_TOKEN_TTL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(15 * 60)
}


fn refresh_ttl_secs() -> u64 {
    std::env::var("REFRESH_TOKEN_TTL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(7 * 24 * 60 * 60)
}


/// Access-token claims carried in the `token` request header.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderToken {
    pub unique_id: String,
    pub exp: usize,
    #[serde(rename = "type", default = "default_access_type")]
    pub token_type: String,
}


fn default_access_type() -> String {
    "access".to_string()
}


impl HeaderToken {
    pub fn new(unique_id: String) -> Self {
        Self {
            unique_id,
            exp: (unix_now() + access_ttl_secs()) as usize,
            token_type: "access".to_string(),
        }
    }

    pub fn get_key() -> Result<String, NanoServiceError> {
        std::env::var("JWT_SECRET").map_err(|e| {
            NanoServiceError::new(
                e.to_string(), 
                NanoServiceErrorStatus::Unauthorized
            )
        })
    }    
    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(error) => Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        }
    }    
    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = DecodingKey::from_secret(key_str.as_ref());
        let validation = Validation::new(Algorithm::HS256);
    
        match decode::<Self>(token, &key, &validation) {
            Ok(token_data) => {
                if token_data.claims.token_type != "access" {
                    return Err(NanoServiceError::new(
                        "invalid access token type".to_string(),
                        NanoServiceErrorStatus::Unauthorized
                    ));
                }
                Ok(token_data.claims)
            },
            Err(error) => Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        }
    }
}


/// Refresh-token claims. Validated against the `refresh_tokens` table on use.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshToken {
    pub unique_id: String,
    pub jti: String,
    pub exp: usize,
    #[serde(rename = "type")]
    pub token_type: String,
}


impl RefreshToken {
    pub fn new(unique_id: String, jti: String) -> Self {
        Self {
            unique_id,
            jti,
            exp: (unix_now() + refresh_ttl_secs()) as usize,
            token_type: "refresh".to_string(),
        }
    }

    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key_str = HeaderToken::get_key()?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(error) => Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        }
    }

    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key_str = HeaderToken::get_key()?;
        let key = DecodingKey::from_secret(key_str.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Self>(token, &key, &validation) {
            Ok(token_data) => {
                if token_data.claims.token_type != "refresh" {
                    return Err(NanoServiceError::new(
                        "invalid refresh token type".to_string(),
                        NanoServiceErrorStatus::Unauthorized
                    ));
                }
                Ok(token_data.claims)
            },
            Err(error) => Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        }
    }

    pub fn expires_at(&self) -> i64 {
        self.exp as i64
    }
}


/// Access + refresh token pair returned by login/refresh endpoints.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthTokenPair {
    pub access_token: String,
    pub refresh_token: String,
}


// Actix Web implementation of FromRequest for HeaderToken
#[cfg(feature = "actix")]
mod actix_impl {
    use super::HeaderToken;
    pub use actix_web::{
        dev::Payload,
        FromRequest as ActixFromRequest,
        HttpRequest,
    };
    use futures::future::{Ready, ok, err};
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus}; 


    impl ActixFromRequest for HeaderToken {
        type Error = NanoServiceError;
        type Future = Ready<Result<HeaderToken, NanoServiceError>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("token") {
                Some(data) => data.to_str().expect("convert token to str"),
                None => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token'".to_string()
                    })
                }
            };
            let token = match HeaderToken::decode(raw_data) {
                Ok(token) => token,
                Err(_) => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not a valid string".to_string()
                    })
                }
            };
            ok(token)
        }
    }
}

// Rocket implementation of FromRequest for HeaderToken
#[cfg(feature = "rocket")]
mod rocket_impl {
    use super::HeaderToken;
    pub use rocket::request::{self, FromRequest as RocketFromRequest, Request};
    use rocket::outcome::Outcome;
    use rocket::http::Status;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};


    #[rocket::async_trait]
    impl<'r> RocketFromRequest<'r> for HeaderToken {
        type Error = NanoServiceError;

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            match req.headers().get_one("token") {
                Some(token) => {
                    let decoded_token = match HeaderToken::decode(token) {
                        Ok(decoded_token) => decoded_token,
                        Err(error) => {
                            return Outcome::Error((Status::Unauthorized, error))
                        }
                    };
                    Outcome::Success(decoded_token)
                },
                None => Outcome::Error((
                    Status::Unauthorized,
                    NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token'".to_string(),
                    },
                )),
            }
        }
    }
}

// Axum implementation of FromRequestParts for HeaderToken
#[cfg(feature = "axum")]
mod axum_impl {
    use super::HeaderToken;
    pub use axum::extract::FromRequestParts as AxumFromRequestParts;
    use axum::http::request::Parts;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};


    impl<S> AxumFromRequestParts<S> for HeaderToken
    where
        S: Send + Sync,
    {
        type Rejection = NanoServiceError;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
            let raw_data = match parts.headers.get("token") {
                Some(data) => data,
                None => {
                    return Err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "Token not found in header under key 'token'".to_string(),
                    });
                }
            };

            let raw_token = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return Err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "Token is not a valid string".to_string(),
                    });
                }
            };

            Ok(HeaderToken::decode(&raw_token)?)
        }
    }
}


#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;

#[cfg(feature = "rocket")]
pub use rocket_impl::RocketFromRequest;

#[cfg(feature = "axum")]
pub use axum_impl::AxumFromRequestParts;
