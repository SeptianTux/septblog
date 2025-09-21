pub fn check(
    config: &std::sync::Arc<json::JsonValue>,
    req: &actix_web::HttpRequest
) -> Result<bool, crate::error::Error> {
    let credentials = match crate::utils::user::check_credentials(&config, &req) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 39,
                    message: err.message
                }
            );
        }
    };

    Ok(credentials)
}