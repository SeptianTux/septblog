pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    config: std::sync::Arc<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if !v {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 77.into();
                ret["error_message"]    = json::JsonValue::String(String::from("You don't have credentials to access this endpoint."));

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );    
            }
        } 
        Err(e) => {
            ret["response"]         = json::JsonValue::Boolean(false);
            ret["error"]            = json::JsonValue::Boolean(true);
            ret["error_code"]       = e.code.into();
            ret["error_message"]    = json::JsonValue::String(e.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );    
        }
    }

    let res = crate::viewmodel::admin_article_editor_get_article_categories_view_model::get(&pool);

    let vec = match res {
        Ok(v) => v,
        Err(e) => {
            ret["response"]         = json::JsonValue::Boolean(false);
            ret["error"]            = json::JsonValue::Boolean(true);
            ret["error_code"]       = e.code.into();
            ret["error_message"]    = json::JsonValue::String(e.message);

            return actix_web::HttpResponse::Ok().body(
                json::stringify_pretty(ret, 4)
            );
        }
    };

    let mut categories = json::JsonValue::new_array();

    for i in &vec {
        match categories.push(i.as_str()) {
            Ok(_) => (),
            Err(_) => {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 31.into();
                ret["error_message"]    = json::JsonValue::String(String::from("Failed to convert data to JSON."));

                return actix_web::HttpResponse::Ok().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    }

    ret["response"]         = json::JsonValue::Boolean(true);
    ret["data"]             = categories;

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}