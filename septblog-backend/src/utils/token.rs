#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
   pub email: String,
   pub exp: u64
}

pub fn get_access_token_from_header(
    req: &actix_web::HttpRequest
) -> Result<Option<String>, crate::error::Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Ok(Some(token.to_owned()));
            }
        }
    }

    Ok(None)
}

pub fn get_email_from_token(
    config: &std::sync::Arc<json::JsonValue>,
    token: &String
) -> Result<Option<String>, crate::error::Error> {
    let secret_key= config["jwt_secret_key"].to_string();
    let claims = match crate::utils::token::decode_token(&token, &secret_key) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 19,
                    message: err.message
                }
            );
        }
    };

    Ok(Some(claims.email))
}

pub fn encode_token(
    claims: &Claims,
    secret_key: &String
) -> Result<String, crate::error::Error> {
    let tok = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secret_key.as_bytes()
        )
    );

    match tok {
        Ok(val) => {
            return Ok(val);
        }
        Err(err) => {
            log::error!("Failed to generate token.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 39,
                    message: String::from("Failed to generate token.")
                }
            )
        }
    }
}

pub fn decode_token(
    token: &String,
    secret_key: &String
) -> Result<Claims, crate::error::Error> {
    let token_message = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret_key.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256)
    );

    let claims = match token_message {
        Ok(ok) => ok.claims,
        Err(err) => {
            log::error!("Error.");
            log::debug!("{:?}", err);

            match err.into_kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    return Err(
                        crate::error::Error {
                            code: 60,
                            message: String::from("Token expired.")
                        }
                    );
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    return Err(
                        crate::error::Error {
                            code: 61,
                            message: String::from("Invalid token.")
                        }
                    );
                }
                _ => {
                    return Err(
                        crate::error::Error {
                            code: 62,
                            message: String::from("Failed to get claims.")
                        }
                    );
                }
            }
        }
    };

    Ok(claims)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_email_from_token() {
        let config = json::parse(&std::fs::read_to_string("./config.json").unwrap()).unwrap();
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6Im1lQHNlcHRpYW4uaWQifQ.C8BOhkN__r80ZcCBTc-R4P6Ab2z0_GAEHIpbcQRdU2Y";
        let arc = std::sync::Arc::new(config);

        let email = super::get_email_from_token(&arc, &token.to_string()).unwrap().unwrap();

        assert_eq!(email, "me@septian.id");
    }
}