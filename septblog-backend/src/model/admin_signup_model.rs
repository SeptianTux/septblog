use mysql::*;
use mysql::prelude::*;
use argon2::PasswordHasher;

pub fn is_username_already_used(pool: &std::sync::Arc<mysql::Pool>, username: &String) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 32,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(u32,)> = match conn.exec_first(
        "SELECT id FROM users WHERE username = :username",
        params! {
            "username" => username
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 33,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    Ok(result.is_some())
}

pub fn is_email_address_already_used(pool: &std::sync::Arc<mysql::Pool>, email_address: &String) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 34,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(u32,)> = match conn.exec_first(
        "SELECT id FROM users WHERE email = :email_address",
        params! {
            "email_address" => email_address
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 35,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    Ok(result.is_some())
}

pub fn insert_user_to_database(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::admin_signup_view::Data
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };
    let now = crate::utils::time::current_unix_timestamp();

    let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
    let argon2 = argon2::Argon2::default();
    let password_hash = match argon2.hash_password(data.password1.as_ref().unwrap().as_bytes(), &salt) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to hash password.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 738,
                    message: "Failed to hash password.".to_string()
                }
            )
        }
    };

    let result = conn.exec_drop(
        r"INSERT INTO users (
                                    username,
                                    password,
                                    email,
                                    first_name,
                                    last_name,
                                    level,
                                    status,
                                    last_login,
                                    created,
                                    deleted
                                ) VALUES (
                                    :username,
                                    :password,
                                    :email,
                                    :first_name,
                                    :last_name,
                                    :level,
                                    :status,
                                    :last_login,
                                    :created,
                                    :deleted
                                )",
        params! {
            "username" => &data.username,
            "password" => password_hash.to_string(),
            "email" => &data.email,
            "first_name" => &data.first_name,
            "last_name" => &data.last_name,
            "level" => 1,
            "status" => 0,
            "last_login" => now,
            "created" => now,
            "deleted" => 0
        }
    );

    match result {
        Ok(_) => {
            ()
        }
        Err(e) => {
            let err = crate::error::Error {
                code: 31,
                message: "Failed to insert data to database.".to_string()
            };
            
            log::error!("Failed to insert data to database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert_user_to_database() {
        let data = crate::view::admin_signup_view::Data {
            username: Some("septian".to_string()),
            password1: Some("123456".to_string()),
            password2: Some("123456".to_string()),
            email: Some("me@septian.id".to_string()),
            first_name: Some("Septian".to_string()),
            last_name: Some("Dwi Cahya".to_string())
        };

        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let res = super::insert_user_to_database(&pool_data, &data);

        assert_eq!(res, Ok(true));
    }

    #[test]
    fn insert_user_to_database_bulk() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);

        for i in (0..200).rev() {
            let mut username = String::new();
            let mut password1 = String::new();
            let mut password2 = String::new();
            let mut email = String::new();
            let mut first_name = String::new();
            let mut last_name = String::new();

            username.push_str(random_word::get(random_word::Lang::En));
            username.push_str(random_word::get(random_word::Lang::En));
            password1.push_str(random_word::get(random_word::Lang::En));
            password1.push_str(random_word::get(random_word::Lang::En));
            password2.push_str(random_word::get(random_word::Lang::En));
            password2.push_str(random_word::get(random_word::Lang::En));
            email.push_str(random_word::get(random_word::Lang::En));
            email.push_str(random_word::get(random_word::Lang::En));
            email.push_str("@gmail.com");
            first_name.push_str(i.to_string().as_str());
            last_name.push_str(random_word::get(random_word::Lang::En));

            let data = crate::view::admin_signup_view::Data {
                username: Some(username),
                password1: Some(password1),
                password2: Some(password2),
                email: Some(email),
                first_name: Some(first_name),
                last_name: Some(last_name)
            };

            let res = super::insert_user_to_database(&pool_data, &data);

            match res {
                Ok(val) => {
                    println!("{:?}", val);
                },
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }

        assert!(true);
    }

    #[test]
    fn is_username_already_used() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let database_pool_arc = std::sync::Arc::new(pool);
        let res = super::is_username_already_used(&database_pool_arc, &"supeno".to_string());

        assert_eq!(res, Ok(false));
    }

    #[test]
    fn is_email_address_already_used() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let database_pool_arc = std::sync::Arc::new(pool);
        let res = super::is_email_address_already_used(&database_pool_arc, &"me@suparjo.id".to_string());

        assert_eq!(res, Ok(false));
    }
}