pub fn already_installed(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let already_installed = match crate::model::already_installed_model::already_installed(&pool) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 483,
                    message: err.message
                }
            )
        }
    };

    Ok(already_installed)
}