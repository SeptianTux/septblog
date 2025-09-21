use mysql::prelude::Queryable;

pub fn get_site_tagline(
    pool: &std::sync::Arc<mysql::Pool>
) -> Result<Option<String>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 278,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let result: Result<Option<(String,)>, mysql::Error> = conn.query_first(
        "SELECT site_tagline FROM settings"
    );

    let res = match result {
        Ok(val) => {
            match val {
                Some(v) => Some(v.0),
                None => None
            }
        }
        Err(err) => {
            log::error!("Failed to get settings data from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 347,
                    message: "Failed to get settings data from database.".to_string()
                }
            );
        }
    };

    Ok(res)
}