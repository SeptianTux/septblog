pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>
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
                ret["error_message"]    = String::from("You don't have credentials to access this endpoint.").into();

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

    match crate::viewmodel::admin_profile_get_view_model::get(&pool, &req, &config) {
        Ok(val) => {
            match val {
                Some(v) => {
                    data["id"]          = v.id.into();
                    data["avatar"]      = v.avatar.into();
                    data["username"]    = v.username.into();
                    data["first_name"]  = v.first_name.into();
                    data["last_name"]   = v.last_name.into();
                    data["email"]       = v.email.into();
                    data["about"]       = v.about.into();
                }
                None => ()
            }
        }
        Err(err) => {
            ret["response"]             = false.into();
            ret["error"]                = true.into();
            ret["error_code"]           = err.code.into();
            ret["error_message"]        = err.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            ); 
        }
    }

    ret["response"]     = true.into();
    ret["data"]         = data;

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}