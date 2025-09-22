use mysql::prelude::Queryable;
use mysql::params;

use argon2::PasswordVerifier;

/*
    Verify credential. It will take a look to database to check if given username and password combination is exist.
    Will return true if given username and password combination exist, false if its not, and return crate::error::Error if
    we have error.
 */
pub fn verify_credencials(
    pool: &std::sync::Arc<mysql::Pool>,
    email: &String,
    password: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 20,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result: Option<(String,)> = match conn.exec_first(
        "SELECT password FROM users WHERE email = :email",
        params! {
            "email"     => email
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 21,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    let password_hash = match result {
        Some(val) => val.0,
        None => {
            return Ok(false);
        }
    };

    let parsed_hash = match argon2::PasswordHash::new(&password_hash) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get password hash.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 748,
                    message: "Failed to get password hash.".to_string()
                }
            )
        }
    };
    let is_valid = argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_credencials() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let email       = String::from("me@septian.id");
        let password    = String::from("1234567");

        let vc = super::verify_credencials(&db_pool, &email, &password);

        let yay = match vc {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("{}", err.message);
                false
            }
        };

        assert_eq!(yay, false);
    }
}