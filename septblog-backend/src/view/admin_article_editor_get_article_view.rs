pub fn get(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest,
    article_id: String
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

            return actix_web::HttpResponse::Unauthorized().body(
                json::stringify_pretty(ret, 4)
            );    
        }
    }

    match crate::viewmodel::admin_article_editor_get_article_view_model::am_i_have_credentials_to_get_this_article(&pool, &config, &req, &article_id) {
        Ok(val) => {
            if !val {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 65.into();
                ret["error_message"]    = json::JsonValue::String(String::from("You don't have credentials to get this article"));

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

            if e.code == 60 {
                return actix_web::HttpResponse::NotFound().body(
                    json::stringify_pretty(ret, 4)
                );
            } else if e.code == 28 {
                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            }

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );
        }
    }

    let article = match crate::viewmodel::admin_article_editor_get_article_view_model::get(pool, article_id) {
        Ok(val) => val,
        Err(e) => {
            ret["response"]         = json::JsonValue::Boolean(false);
            ret["error"]            = json::JsonValue::Boolean(true);
            ret["error_code"]       = e.code.into();
            ret["error_message"]    = json::JsonValue::String(e.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );
        }
    };

    match article {
        Some(val) => {
            let mut data = json::JsonValue::new_object();
            let mut categories = json::JsonValue::new_array();

            ret["response"]         = json::JsonValue::Boolean(true);

            data["id"]              = json::JsonValue::String(val.id);
            data["title"]           = json::JsonValue::String(val.title);
            data["author"]          = val.author.into();
            data["content"]         = json::JsonValue::String(val.content);
            data["status"]          = val.status.into();

            for i in &val.categories {
                match categories.push(i.as_str()) {
                    Ok(_) => (),
                    Err(_) => {
                        ret["response"]         = json::JsonValue::Boolean(false);
                        ret["error"]            = json::JsonValue::Boolean(true);
                        ret["error_code"]       = 36.into();
                        ret["error_message"]    = json::JsonValue::String(String::from("Failed to convert data to JSON."));

                        return actix_web::HttpResponse::Ok().body(
                            json::stringify_pretty(ret, 4)
                        );
                    }
                }
            }

            data["categories"]      = categories;

            ret["data"]             = data;
        }
        None => {
            ret["response"]         = json::JsonValue::Boolean(true);
            ret["data"]             = json::JsonValue::Null;

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );
        }
    }

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}