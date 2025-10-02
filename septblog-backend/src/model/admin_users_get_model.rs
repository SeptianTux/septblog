use mysql::prelude::Queryable;
use mysql::params;

struct UserFromDatabase {
    id: u64,
    avatar: Option<String>,
    first_name: String,
    last_name: Option<String>,
    username: String,
    created: u64,
    level: u8,
    status: u8
}

pub fn get_users_data_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    page: &u64
) -> Result<Vec<crate::viewmodel::admin_users_get_view_model::User>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            let e = crate::error::Error {
                code: 32,
                message: "Failed to get pooled database connection.".to_string()
            };

            return Err(e);
        }
    };

    let mut ret: Vec<crate::viewmodel::admin_users_get_view_model::User> = Vec::new();

    let per_page = 20;
    let offset = (page - 1) * per_page;

    let selected_users = conn.exec_map(
        "SELECT id, avatar, first_name, last_name, username, created, level, status FROM users WHERE deleted=0 ORDER BY id DESC LIMIT :limit OFFSET :offset  ",
        params! {
            "limit" => per_page,
            "offset" => offset
        },
        |(id, avatar, first_name, last_name, username, created, level, status)| UserFromDatabase {
            id,
            avatar,
            first_name,
            last_name,
            username,
            created,
            level,
            status
        },
    );

    let users_from_database: Vec<UserFromDatabase> = match selected_users {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get articles from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 29,
                    message: String::from("Failed to get articles from database.")
                }
            );
        }
    };

    for i in users_from_database {
        let result: Option<(u64,)> = match conn.exec_first(
            "SELECT COUNT(*) FROM articles WHERE author = :author",
            params! {
                "author"     => i.id
            },
        ) {
            Ok(res) => res,
            Err(e) => {
                log::error!("Error getting data from database.");
                log::debug!("{:?}", e);

                let err: crate::error::Error = crate::error::Error {
                    code: 982,
                    message: String::from("Error getting data from database.")
                };

                return Err(err);
            }
        };

        let article_count = match result {
            Some(v) => v.0,
            None => 0
        };

        let mut user = crate::viewmodel::admin_users_get_view_model::User {
            id: i.id,
            avatar: i.avatar,
            first_name: i.first_name,
            last_name: i.last_name,
            username: i.username,
            articles: article_count,
            created: i.created,
            level: i.level,
            status: i.status
        };

        if user.avatar.is_none() {
            user.avatar = Some(String::from("/uploads/user.png"));
        }

        ret.push(user);

    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_users_data_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let page: u64 = 1;

        let res = super::get_users_data_from_database(&pool_data, &page).unwrap();

        println!("{:#?}", res);

        assert_ne!(res.len(), 0);
    }
}