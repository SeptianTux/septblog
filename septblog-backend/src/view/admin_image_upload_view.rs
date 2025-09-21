pub async fn upload(
    payload: actix_multipart::Multipart,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();

    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if v == false {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 81.into();
                ret["error_message"]    = json::JsonValue::String(String::from("You don't have credentials to access this endpoint."));

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
        Err(e) => {
            ret["response"]             = json::JsonValue::Boolean(false);
            ret["error"]                = json::JsonValue::Boolean(true);
            ret["error_code"]           = 82.into();
            ret["error_message"]        = json::JsonValue::String(e.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            ); 
        }
    }

    match crate::viewmodel::admin_image_upload_view_model::upload(payload).await {
        Ok(val) => {
            ret["response"]                 = json::JsonValue::Boolean(true);
            ret["url"]                      = json::JsonValue::String(val);

            return actix_web::HttpResponse::Created().body(
                json::stringify_pretty(ret, 4)
            ); 
        }
        Err(err) => {
            ret["response"]                 = json::JsonValue::Boolean(false);
            ret["error"]                    = json::JsonValue::Boolean(true);
            ret["error_code"]               = err.code.into();
            ret["error_message"]            = json::JsonValue::String(err.message);

            if err.code == 50 || err.code == 52 || err.code == 54 {
                return actix_web::HttpResponse::BadRequest().body(
                    json::stringify_pretty(ret, 4)
                );  
            } else {
                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    }
}