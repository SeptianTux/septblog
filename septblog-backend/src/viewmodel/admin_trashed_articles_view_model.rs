#[derive(Debug)]
pub struct TrashedArticle {
    pub id: String,
    pub title: String,
    pub author_username: String,
    pub author_first_name: String,
    pub author_last_name: Option<String>,
    pub visitors: u64,
    pub created: u64
}

pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
    page: &u32
) -> Result<Vec<TrashedArticle>, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 899,
                            message: "Invalid credentials.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 892,
                    message: err.message
                }
            );
        }
    };

    let res = match crate::model::admin_trashed_articles_model::get_trashed_article_data(&pool, &user_id, &page) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 698,
                    message: err.message
                }
            );
        }
    };

    Ok(res)
}