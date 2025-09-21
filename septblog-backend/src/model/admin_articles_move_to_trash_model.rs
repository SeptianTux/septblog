use mysql::{params, prelude::Queryable};

pub fn move_to_trash(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 478,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let query = "UPDATE articles SET status=3 WHERE id = :id";
    let params = params! {
        "id" => article_id
    };
    let res = conn.exec_drop(query, params);

    match res {
        Ok(_val) => {
            return Ok(true)
        }
        Err(err) => {
            log::error!("Failed to move article to trash.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 499,
                    message: "Failed to move article to trash.".to_string()
                }
            )
        }
    }
}