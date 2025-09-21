use mysql::prelude::Queryable;
use mysql::params;

struct TrashedArticleData {
    id: String,
    title: String,
    author: u64,
    created: u64
}

pub fn get_trashed_article_data(
    pool: &std::sync::Arc<mysql::Pool>,
    author: &u64,
    page: &u32
) -> Result<Vec<crate::viewmodel::admin_trashed_articles_view_model::TrashedArticle>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 198,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let per_page = 20;
    let offset = (page - 1) * per_page;

    let articles = conn.exec_map(
        "SELECT id, title, author, created FROM articles WHERE author = :article_author and status=3 ORDER BY counter DESC LIMIT :limit OFFSET :offset",
        params! {
            "article_author" => author,
            "limit" => per_page,
            "offset" => offset
        },
        |(id, title, author, created)| TrashedArticleData {
            id,
            title,
            author,
            created
        },
    );

    let articless = match articles {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get articles from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 478,
                    message: String::from("Failed to get articles from database.")
                }
            );
        }
    };

    let mut ret: Vec<crate::viewmodel::admin_trashed_articles_view_model::TrashedArticle> = Vec::new();

    for i in articless {
        let result: Option<(u64,)> = match conn.exec_first(
            "SELECT COUNT(*) FROM article_viewers WHERE article_id = :article_id",
            params! {
                "article_id" => i.id.clone(),
            },
        ) {
            Ok(res) => res,
            Err(e) => {
                log::error!("Error getting article viewers data from database.");
                log::debug!("{:?}", e);

                let err: crate::error::Error = crate::error::Error {
                    code: 899,
                    message: String::from("Error getting article viewers data from database.")
                };

                return Err(err);
            }
        };
        let visitors = match result {
            Some(val) => val.0,
            None => 0
        };
        let result: Option<(String, String, Option<String>,)> = match conn.exec_first(
            "SELECT username, first_name, last_name FROM users WHERE id = :id",
            params! {
                "id"     => i.author,
            },
        ) {
            Ok(res) => res,
            Err(e) => {
                log::error!("Error getting user data from database.");
                log::debug!("{:?}", e);

                let err: crate::error::Error = crate::error::Error {
                    code: 619,
                    message: String::from("Error getting user data from database.")
                };

                return Err(err);
            }
        };
        let author = match result {
            Some(val) => val,
            None => (String::new(), String::new(), None)
        };

        let article = crate::viewmodel::admin_trashed_articles_view_model::TrashedArticle {
            id: i.id,
            title: i.title,
            author_username: author.0,
            author_first_name: author.1,
            author_last_name: author.2,
            visitors: visitors,
            created: i.created
        };

        ret.push(article);
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_trashed_article_data() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let author = 1;
        let page = 1;

        let res = super::get_trashed_article_data(&db_pool, &author, &page);

        println!("{:#?}", res);

        assert!(res.is_ok());
    }
}