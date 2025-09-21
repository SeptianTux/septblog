use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub author: i32,
    pub content: String,
    pub status: i32
}

pub struct ArticleCategories {
    pub category_id: String
}

pub fn get_category_name_by_id(
    pool: &actix_web::web::Data<mysql::Pool>,
    category_id: &String
) -> Result<Option<String>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 89,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(String,)> = match conn.exec_first(
        "SELECT name FROM categories WHERE id = :id",
        params! {
            "id"     => category_id,
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error getting category name from database.");
            log::debug!("{:?}", e);

            return Err(
                crate::error::Error {
                    code: 76,
                    message: "Error getting category name from database.".to_string()
                }
            );
        }
    };

    let category_name_option = match result {
        Some((s,)) => Some(s),
        None => None
    };

    Ok(category_name_option)
}

pub fn get_article(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String
) -> Result<Option<Article>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 69,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let res: Result<Option<(String, String, i32, String, i32)>> = conn.exec_first(
        "SELECT id, title, author, content, status FROM articles WHERE id = :id",
        params! {
            "id" => &article_id,
        },
    );
    let res_option = match res {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to get article from database");
            log::debug!("{:?}", e);

            return Err(
                crate::error::Error {
                    code: 38,
                    message: "Failed to get article from database".to_string()
                }
            );
        }
    };

    let article_option = res_option.map(|(id, title, author, content, status)| Article { id, title, author, content, status });
    let article = match article_option {
        Some(v) => v,
        None => {
            return Ok(None);
        }
    };

    Ok(Some(article))
}

pub fn get_article_categories(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String
) -> Result<Vec<ArticleCategories>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 69,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result_articles_categories = conn
        .exec_map(
            "SELECT category_id FROM article_categories WHERE article_id = :article_id",
            params! {
                "article_id" => &article_id,
            },
            | category_id | ArticleCategories { category_id },
    );

    let ret = match result_articles_categories {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get article categories.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 341,
                    message: "Failed to get article categories".to_string()
                }
            );
        }
    };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_category_name_by_id() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let category_id = String::from("test");
        let ret = crate::model::admin_article_editor_get_article_model::get_category_name_by_id(&pool_data, &category_id);

        let category_name = match ret {
            Ok(val) => {
                match val {
                    Some(v) => v,
                    None => String::new()
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                String::new()
            }
        };

        assert_eq!(category_name, String::from("Rock"));
    }
}