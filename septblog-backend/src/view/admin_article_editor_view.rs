#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Data {
    pub article_id: Option<String>,
    pub article_title: String,
    pub article_content: String,
    pub article_categories: Vec<String>,
    pub article_status: i8
}

fn validation(data: &Data) -> Result<bool, crate::error::Error> {
    let mut error: bool = false;
    let mut error_message: String = String::new();

    if data.article_title.is_empty() {
        error = true;
        error_message.push_str("Article title is empty.");
    } else if data.article_content.is_empty() {
        error = true;
        error_message.push_str("Article content is empty.");
    }

    if error {
        return Err(
            crate::error::Error {
                code: 13,
                message: error_message
            }
        );
    }

    Ok(true)
}

pub fn save(
    pool: std::sync::Arc<mysql::Pool>,
    config: std::sync::Arc<json::JsonValue>,
    data: Data,
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

    match validation(&data) {
        Ok(_) => { () }
        Err(err) => {
            res["response"]         = json::JsonValue::Boolean(false);
            res["error"]            = json::JsonValue::Boolean(true);
            res["error_code"]       = err.code.into();
            res["error_message"]    = json::JsonValue::String(err.message);

            return actix_web::HttpResponse::BadRequest().body(
                json::stringify_pretty(res, 4)
            );
        }        
    };

    let ret = match crate::viewmodel::admin_article_editor_view_model::save(pool.into(), req, config, data) {
        Ok(ok) => ok,
        Err(e) => {
            if e.code == 68 {
                res["response"]     = json::JsonValue::Boolean(false);
                res["message"]      = json::JsonValue::String(String::from("Unauthorized."));
                
                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(res, 4)
                );
            } else {
                res["response"]         = json::JsonValue::Boolean(false);
                res["error"]            = json::JsonValue::Boolean(true);
                res["error_code"]       = e.code.into();
                res["error_message"]    = json::JsonValue::String(e.message);

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(res, 4)
                );
            }
        }
    };

    res["response"]     = json::JsonValue::Boolean(true);
    res["article_id"]   = json::JsonValue::String(ret);

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(res, 4)
    );
}