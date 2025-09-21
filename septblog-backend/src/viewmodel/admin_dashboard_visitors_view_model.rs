#[derive(Debug)]
pub struct Visitor {
    pub article_title: String,
    pub article_id: String,
    pub article_author: u64,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub visited_at: u64
}

pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>
) -> Result<Vec<Visitor>, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 900,
                            message: "Invalid credentials.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 219,
                    message: err.message
                }
            );
        }
    };
    let visitors = match crate::model::admin_dashboard_visitors_model::get_visitors_from_database(&pool, &user_id) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 827,
                    message: err.message
                }
            );
        }
    };
    
    Ok(visitors)
}