pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
    page: u32
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
    
    let articles = match crate::viewmodel::admin_articles_view_model::get(&pool, &req, &config, &page) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            if err.code == 889 {
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

    for i in articles {
        let mut article = json::JsonValue::new_object();

        article["id"]                   = i.id.into();
        article["title"]                = i.title.into();
        article["author_username"]      = i.author_username.into();
        article["author_first_name"]    = i.author_first_name.into();
        article["author_last_name"]     = i.author_last_name.into();
        article["visitors"]             = i.visitors.into();
        article["status"]               = i.status.into();
        article["created"]              = i.created.into();

        match data.push(article) {
            Ok(_) => (),
            Err(err) => {
                log::error!("{:?}", err);

                ret["response"]         = false.into();
                ret["error"]            = true.into();
                ret["error_code"]       = 736.into();
                ret["error_message"]    = String::from("Failed to push article to an array.").into();

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );    
            }
        }
    }

    let mut has_more = false;
    
    if data.len() > 0 {
        has_more = true;
    }

    ret["response"]     = true.into();
    ret["data"]         = data;
    ret["has_more"]     = has_more.into();

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );    
}