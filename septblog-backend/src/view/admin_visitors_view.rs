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

    let visitors = match crate::viewmodel::admin_visitors_view_model::get(&pool, &req, &config, &page) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            if err.code == 293 {
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

    let mut has_more = false;
    if visitors.len() > 0 {
        has_more = true;
    }

    for i in visitors {
        let mut visitor = json::JsonValue::new_object();

        visitor["article_title"]    = i.article_title.into();
        visitor["article_id"]       = i.article_id.into();
        visitor["article_author"]   = i.article_author.into();
        visitor["ip_address"]       = i.ip_address.into();
        visitor["user_agent"]       = i.user_agent.into();
        visitor["referer"]          = i.referer.into();
        visitor["visited_at"]       = i.visited_at.into();

        match data.push(visitor) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{:?}", err);

                ret["response"]         = false.into();
                ret["error"]            = true.into();
                ret["error_code"]       = 892.into();
                ret["error_message"]    = String::from("Failed to push visitor's data to an array.").into();

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    }

    ret["response"]     = true.into();
    ret["data"]         = data;
    ret["has_more"]     = has_more.into();

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );
}