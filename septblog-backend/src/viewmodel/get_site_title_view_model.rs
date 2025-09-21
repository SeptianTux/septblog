pub fn get(pool: &std::sync::Arc<mysql::Pool>) -> Result<Option<String>, crate::error::Error> {
    let ret = match crate::model::get_site_title_model::get_site_title(&pool) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 326,
                    message: err.message
                }
            );
        }
    };

    Ok(ret)
}