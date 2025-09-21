pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
    start: &u64,
    end: &u64
) -> Result<u64, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 329,
                            message: "Invalid credentials.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 390,
                    message: err.message
                }
            );
        }
    };

    let res = match crate::model::admin_dashboard_chart_model::get_data_from_database(&pool, &user_id, &start, &end) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 983,
                    message: err.message
                }
            );
        }
    };

    Ok(res)
}