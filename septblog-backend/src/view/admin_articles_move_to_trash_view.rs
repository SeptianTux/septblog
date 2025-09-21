pub fn move_to_trash(
    pool: std::sync::Arc<mysql::Pool>,
    config: std::sync::Arc<json::JsonValue>,
    article_id: String,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    let mut res = json::JsonValue::new_object();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if !v {
                res["response"]         = json::JsonValue::Boolean(false);
                res["error"]            = json::JsonValue::Boolean(true);
                res["error_code"]       = 12.into();
                res["error_message"]    = json::JsonValue::String(String::from("Unauthorized. You don't have credentials to access this endpoint."));

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(res, 4)
                );
            }
        }
        Err(e) => {
            res["response"]         = json::JsonValue::Boolean(false);
            res["error"]            = json::JsonValue::Boolean(true);
            res["error_code"]       = 13.into();
            res["error_message"]    = json::JsonValue::String(format!("Error while checking credentials. Error message: {}.", e.message));

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(res, 4)
            );
        }
    }

    let move_to_trash = match crate::viewmodel::admin_articles_move_to_trash_view_model::move_to_trash(&pool, &article_id) {
        Ok(val) => val,
        Err(err) => {
            res["response"]         = json::JsonValue::Boolean(false);
            res["error"]            = json::JsonValue::Boolean(true);
            res["error_code"]       = 13.into();
            res["error_message"]    = json::JsonValue::String(err.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(res, 4)
            );
        }
    };

    res["response"]     = json::JsonValue::Boolean(move_to_trash);

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(res, 4)
    )
}