pub fn it_is_already_installed(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let already_installed = match crate::model::install_model::it_is_already_installed(&pool) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 392,
                    message: err.message
                }
            )
        }
    };

    Ok(already_installed)
}

pub fn install(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::install_view::Data
) -> Result<bool, crate::error::Error> {
    match crate::model::install_model::create_tables(&pool) {
        Ok(_) => (),
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 827,
                    message: err.message
                }
            )
        }
    };
    match crate::model::install_model::insert_data_to_database(&pool, &data) {
        Ok(_) => (),
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 398,
                    message: err.message
                }
            )
        }
    };
    match crate::model::install_model::add_hello_world_article(&pool) {
        Ok(_) => (),
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 929,
                    message: err.message
                }
            )
        }
    };
    match crate::model::install_model::set_already_installed_true(&pool) {
        Ok(_) => (),
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 629,
                    message: err.message
                }
            )
        }
    };

    Ok(true)
}