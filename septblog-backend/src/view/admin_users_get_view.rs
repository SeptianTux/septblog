pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
    page: u64
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_array();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if !v {
                ret["response"]         = false.into();
                ret["error"]            = true.into();
                ret["error_code"]       = 77.into();
                ret["error_message"]    = String::from("You don't have credentials to access this endpoint.").into();

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        } 
        Err(e) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = e.code.into();
            ret["error_message"]    = e.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );    
        }
    }

    let user_level = match crate::utils::user::get_user_level(&pool, &req, &config) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    ret["response"]         = false.into();
                    ret["error"]            = true.into();
                    ret["error_code"]       = 98.into();
                    ret["error_message"]    = String::from("You don't have credentials to access this endpoint.").into();

                    log::warn!("The user request aborted because the user don't have credentials.");

                    return actix_web::HttpResponse::Unauthorized().body(
                        json::stringify_pretty(ret, 4)
                    );
                }
            }
        }
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = 198.into();
            ret["error_message"]    = err.message.into();

            if err.code == 819 {
                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            } else {
                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    };

    if user_level != 0 {
        ret["response"]         = false.into();
        ret["error"]            = true.into();
        ret["error_code"]       = 901.into();
        ret["error_message"]    = "Forbidden.".into();

        return actix_web::HttpResponse::Forbidden().body(
            json::stringify_pretty(ret, 4)
        );
    }

    let res = match crate::viewmodel::admin_users_get_view_model::get(&pool, &page) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );    
        }
    };

    let mut has_more = false;
    if res.len() > 0 {
        has_more = true;
    }

    for i in res {
        let mut user = json::JsonValue::new_object();

        user["id"]          = i.id.into();
        user["avatar"]      = i.avatar.into();
        user["first_name"]  = i.first_name.into();
        user["last_name"]   = i.last_name.into();
        user["username"]    = i.username.into();
        user["articles"]    = i.articles.into();
        user["created"]     = i.created.into();
        user["level"]       = i.level.into();
        user["status"]      = i.status.into();

        match data.push(user) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("get() : {:?}", err);

                ret["response"]         = false.into();
                ret["error"]            = true.into();
                ret["error_code"]       = 647.into();
                ret["error_message"]    = "Failed to push an element into an array".to_string().into();

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                ); 
            }
        }
    }

    ret["response"]     = true.into();
    ret["data"]         = data;
    ret["has_more"]     = has_more.into();

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}