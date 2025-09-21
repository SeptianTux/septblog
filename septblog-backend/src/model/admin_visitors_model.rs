use mysql::prelude::Queryable;
use mysql::params;

pub fn get_visitors_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    article_author: &u64,
    page: u32
) -> Result<Vec<crate::viewmodel::admin_visitors_view_model::Visitor>, crate::error::Error> {
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

    let visitors = conn.exec_map(
        "SELECT articles.title, article_viewers.article_id, article_viewers.article_author, article_viewers.ip_address, article_viewers.user_agent, article_viewers.referer, article_viewers.visited_at FROM article_viewers INNER JOIN articles ON article_viewers.article_id=articles.id WHERE article_author = :article_author ORDER BY visited_at DESC LIMIT :limit OFFSET :offset",
        params! {
            "article_author" => article_author,
            "limit" => per_page,
            "offset" => offset
        },
        |(article_title, article_id, article_author, ip_address, user_agent, referer, visited_at)| crate::viewmodel::admin_visitors_view_model::Visitor {
            article_title,
            article_id,
            article_author,
            ip_address,
            user_agent,
            referer,
            visited_at
        },
    );

    let ret = match visitors {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get visitors from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 489,
                    message: String::from("Failed to get visitors from database.")
                }
            );
        }
    };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn admin_visitors_get_visitors_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let article_author = 1;
        let page = 1;

        let result = match super::get_visitors_from_database(&db_pool, &article_author, page) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{:?}", err);
                return;
            }
        };

        println!("{:#?}", result);

        assert_ne!(result.len(), 0);
    }
}