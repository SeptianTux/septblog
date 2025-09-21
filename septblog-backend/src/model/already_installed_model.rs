use mysql::prelude::Queryable;

pub fn already_installed(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 788,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };
    let query = "SHOW TABLES LIKE 'settings'";
    let res: Result<Option<String>, mysql::Error> = conn.query_first(query);
    let table_name = match res {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get table information.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 372,
                    message: "Failed to get table information.".to_string()
                }
            )
        }
    };
    let mut ret = false;

    if table_name.is_some() {
        let query = "SELECT already_installed FROM settings";
        let res: Result<Option<u8>, mysql::Error> = conn.query_first(query);
        let val = match res {
            Ok(val) => {
                match val {
                    Some(v) => v,
                    None => 0
                }
            }
            Err(err) => {
                log::error!("Failed to get already installed value.");
                log::debug!("{:?}", err);

                return Err(
                    crate::error::Error {
                        code: 899,
                        message: "Failed to get already installed value.".to_string()
                    }
                )
            }
        };

        if val > 0 {
            ret = true;
        }
    }

    Ok(ret)
}