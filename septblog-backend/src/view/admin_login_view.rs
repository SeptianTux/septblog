use json::JsonValue;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Data {
    pub email: String,
    pub password: String,
}

pub fn login(
    pool: actix_web::web::Data<mysql::Pool>,
    config: std::sync::Arc<JsonValue>,
    data: Data
) -> impl actix_web::Responder {
    let login = crate::viewmodel::admin_login_view_model::login(&pool.into_inner(), &data.email, &data.password);
    let mut ret = json::JsonValue::new_object();
    let mut json_string = String::new();

    match login {
        Ok(val) => {
            if val {
                let mut dat = json::JsonValue::new_object();
                let access_token = match crate::viewmodel::admin_login_view_model::generate_access_token(&data.email, &config) {
                    Ok(val) => val,
                    Err(err) => {
                        ret["response"]         = json::JsonValue::Boolean(false);
                        ret["error"]            = json::JsonValue::Boolean(true);
                        ret["error_code"]       = err.code.into();
                        ret["error_message"]    = json::JsonValue::String(err.message);

                        json_string.push_str(json::stringify_pretty(ret, 4).as_str());

                        return actix_web::HttpResponse::InternalServerError().body(json_string);
                    }
                };

                dat["access_token"]     = json::JsonValue::String(access_token);

                ret["response"]         = json::JsonValue::Boolean(true);
                ret["login"]            = json::JsonValue::Boolean(true);
                ret["data"]             = dat;


                json_string.push_str(json::stringify_pretty(ret, 4).as_str());

                return actix_web::HttpResponse::Ok().body(json_string);
            } else {
                ret["response"]        = json::JsonValue::Boolean(false);
                ret["login"]           = json::JsonValue::Boolean(false);
                ret["message"]         = json::JsonValue::String(String::from("Invalid email and password combination."));

                json_string.push_str(json::stringify_pretty(ret, 4).as_str());
                
                return actix_web::HttpResponse::Unauthorized().body(json_string)
            }
        }
        Err(e) => {
            ret["response"]            = json::JsonValue::Boolean(false);
            ret["error"]               = json::JsonValue::Boolean(true);
            ret["error_code"]          = 99.into();
            ret["error_message"]       = json::JsonValue::String(e.message);

            json_string.push_str(json::stringify_pretty(ret, 4).as_str());

            return actix_web::HttpResponse::InternalServerError().body(json_string);
        }
    }
}