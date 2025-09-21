pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_array();
    let mut json = String::new();
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

    match crate::viewmodel::admin_dashboard_visitors_view_model::get(&pool, &req, &config) {
        Ok(val) => {
            ret["response"]         = true.into();

            for i in val {
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
                        ret["error_message"]    = String::from("Failed to push data to an array.").into();

                        json.push_str(json::stringify_pretty(ret, 4).as_str());

                        return actix_web::HttpResponse::Unauthorized().body(json);
                    }
                }
            }

            ret["data"] = data;

            json.push_str(json::stringify_pretty(ret, 4).as_str());

            return actix_web::HttpResponse::Ok().body(json);
        }
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_message"]    = err.message.into();

            json.push_str(json::stringify_pretty(ret, 4).as_str());

            if err.code == 650 {
                return actix_web::HttpResponse::Unauthorized().body(json);
            } else {
                return actix_web::HttpResponse::InternalServerError().body(json);
            }
        }
    }
}