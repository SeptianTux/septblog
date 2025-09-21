pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_object();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if v == false {
                ret["response"]         = false.into();
                ret["error"]            = true.into();
                ret["error_code"]       = 81.into();
                ret["error_message"]    = "You don't have credentials to access this endpoint.".into();

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
        Err(e) => {
            ret["response"]             = false.into();
            ret["error"]                = true.into();
            ret["error_code"]           = 82.into();
            ret["error_message"]        = e.message.into();

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

    let res = match crate::viewmodel::admin_settings_get_view_model::get(&pool) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]             = false.into();
            ret["error"]                = true.into();
            ret["error_code"]           = err.code.into();
            ret["error_message"]        = err.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );
        }
    };

    ret["response"]             = true.into();
    
    data["site_title"]          = res.site_title.into();
    data["site_tagline"]        = res.site_tagline.into();
    data["enable_signup_page"]  = res.enable_signup_page.into();

    ret["data"]                 = data.into();

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}