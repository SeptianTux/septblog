#[actix_web::get("/admin/check-credentials")]
async fn admin_check_credentials(
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_check_credentials_view::check(config.into_inner(), req)
}

#[actix_web::put("/admin/signup")]
async fn admin_signup(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_signup_view::Data>
) -> impl actix_web::Responder {
    crate::view::admin_signup_view::signup(pool.into_inner(), config.into_inner(), data.into_inner())
}

#[actix_web::post("/admin/login")]
async fn admin_login(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_login_view::Data>
) -> impl actix_web::Responder {
    crate::view::admin_login_view::login(pool, config.into_inner(), data.into_inner())
}

#[actix_web::put("/admin/article-editor")]
async fn admin_article_editor(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_article_editor_view::Data>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_article_editor_view::save(pool.into_inner(), config.into_inner(), data.into_inner(), req)
}

#[actix_web::get("/admin/article-editor/get-article-categories")]
async fn admin_article_editor_get_article_categories(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_article_editor_get_article_categories_view::get(pool.into_inner(), config.into_inner(), req)
}

#[actix_web::get("/admin/article-editor/get-article/{article_id}")]
async fn admin_article_editor_get_article(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest,
    article_id: actix_web::web::Path<String>
) -> impl actix_web::Responder {
    crate::view::admin_article_editor_get_article_view::get(pool, config, req, article_id.into_inner())
}

#[actix_web::put("/admin/image-upload")]
async fn admin_image_upload(
    payload: actix_multipart::Multipart,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>
) -> impl actix_web::Responder {
    crate::view::admin_image_upload_view::upload(payload, req, config).await
}

#[actix_web::get("/admin/dashboard/chart/{start}/{end}")]
async fn admin_dashboard_chart(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    path: actix_web::web::Path<(u64, u64)>
) -> impl actix_web::Responder {
    let (start, end) = path.into_inner();
    crate::view::admin_dashboard_chart_view::get(pool.into_inner(), req, config.into_inner(), start, end)
}

#[actix_web::get("/admin/dashboard/visitors")]
async fn admin_dashboard_visitors(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>
) -> impl actix_web::Responder {
    crate::view::admin_dashboard_visitors_view::get(pool.into_inner(), req, config.into_inner())
}

#[actix_web::get("/admin/visitors/{page}")]
async fn admin_visitors(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    page: actix_web::web::Path<u32>
) -> impl actix_web::Responder {
    crate::view::admin_visitors_view::get(pool.into_inner(), req, config.into_inner(), page.into_inner())
}

#[actix_web::get("/admin/articles/{page}")]
async fn admin_articles(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    page: actix_web::web::Path<u32>
) -> impl actix_web::Responder {
    crate::view::admin_articles_view::get(pool.into_inner(), req, config.into_inner(), page.into_inner())
}

#[actix_web::get("/admin/trashed-articles/{page}")]
async fn admin_trashed_articles(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    page: actix_web::web::Path<u32>
) -> impl actix_web::Responder {
    crate::view::admin_trashed_articles_view::get(pool.into_inner(), req, config.into_inner(), page.into_inner())
}

#[actix_web::get("/admin/articles/move-to-trash/{article_id}")]
async fn admin_articles_move_to_trash(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    article_id: actix_web::web::Path<String>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_articles_move_to_trash_view::move_to_trash(pool.into_inner(), config.into_inner(), article_id.into_inner(), req)
}

#[actix_web::get("/admin/articles/delete/{article_id}")]
async fn admin_articles_delete(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    article_id: actix_web::web::Path<String>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_trashed_articles_deletion_view::delete(pool.into_inner(), req, config.into_inner(), article_id.into_inner())
}

#[actix_web::get("/admin/profile")]
async fn admin_profile_get(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>
) -> impl actix_web::Responder {
    crate::view::admin_profile_get_view::get(pool.into_inner(), req, config.into_inner())
}

#[actix_web::put("/admin/profile")]
async fn admin_profile_put(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_profile_put_view::ProfileFormData>
) -> impl actix_web::Responder {
    crate::view::admin_profile_put_view::put(pool.into_inner(), req, config.into_inner(), data.into_inner())
}

#[actix_web::get("/admin/users/{page}")]
async fn admin_users_get(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    page: actix_web::web::Path<u64>
) -> impl actix_web::Responder {
    crate::view::admin_users_get_view::get(pool.into_inner(), req, config.into_inner(), page.into_inner())
}

/*
    command :
                1 : Activate
                2 : Suspend
                3 : Delete
*/
#[actix_web::put("/admin/users/{command}/{user_id}")]
async fn admin_users_put(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    path: actix_web::web::Path<(u8, u64)>
) -> impl actix_web::Responder {
    let (command, user_id) = path.into_inner();
    
    crate::view::admin_users_put_view::put(pool.into_inner(), req, config.into_inner(), command, user_id)
}

#[actix_web::get("/admin/settings")]
async fn admin_settings_get(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
) -> impl actix_web::Responder {
    crate::view::admin_settings_get_view::get(pool.into_inner(), req, config.into_inner())
}

#[actix_web::put("/admin/settings")]
async fn admin_settings_put(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_settings_put_view::SettingsFormData>
) -> impl actix_web::Responder {
    crate::view::admin_settings_put_view::put(pool.into_inner(), req, config.into_inner(), data.into_inner())
}

#[actix_web::put("/admin/security/change-password")]
async fn admin_security_change_password(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_security_change_password_view::Data>
) -> impl actix_web::Responder {
    crate::view::admin_security_change_password_view::put(pool.into_inner(), req, config.into_inner(), data.into_inner())
}

#[actix_web::put("/admin/security/change-email")]
async fn admin_security_change_email(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<json::JsonValue>,
    data: actix_web::web::Json<crate::view::admin_security_change_email_view::FormData>
) -> impl actix_web::Responder {
    crate::view::admin_security_change_email_view::put(pool.into_inner(), req, config.into_inner(), data.into_inner())
}

#[actix_web::get("/admin/logged-in-as")]
async fn admin_logged_in_as(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_logged_in_as_view::get(pool.into_inner(), config.into_inner(), req)
}

#[actix_web::get("/admin/get-user-level")]
async fn admin_get_user_level(
    pool: actix_web::web::Data<mysql::Pool>,
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::admin_get_user_level_view::get_user_level(pool.into_inner(), req, config.into_inner())
}


#[actix_web::get("/articles/{page}")]
async fn articles(
    pool: actix_web::web::Data<mysql::Pool>,
    page: actix_web::web::Path<u64>
) -> impl actix_web::Responder {
    crate::view::articles_view::get(pool.into_inner(), page.into_inner())
}

#[actix_web::get("/article/{id}")]
async fn article(
    pool: actix_web::web::Data<mysql::Pool>,
    id: actix_web::web::Path<String>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    crate::view::article_view::get(pool.into_inner(), id.into_inner(), req)
}

#[actix_web::get("/category/{category}/{page}")]
async fn category(
    pool: actix_web::web::Data<mysql::Pool>,
    path: actix_web::web::Path<(String, u64)>
) -> impl actix_web::Responder {
    let (category, page) = path.into_inner();
    
    crate::view::category_view::get_article_with_this_category_id(pool.into_inner(), category, page)
}

#[actix_web::get("/user/{username}")]
async fn user(
    pool: actix_web::web::Data<mysql::Pool>,
    username: actix_web::web::Path<String>
) -> impl actix_web::Responder {
    crate::view::user_view::get(pool.into_inner(), username.into_inner())
}

#[actix_web::get("/get-site-title")]
async fn get_site_title(
    pool: actix_web::web::Data<mysql::Pool>
) -> impl actix_web::Responder {
    crate::view::get_site_title_view::get(pool.into_inner())
}

#[actix_web::get("/get-site-tagline")]
async fn get_site_tagline(
    pool: actix_web::web::Data<mysql::Pool>
) -> impl actix_web::Responder {
    crate::view::get_site_tagline_view::get(pool.into_inner())
}

#[actix_web::get("/get-full-year")]
async fn get_full_year() -> impl actix_web::Responder {
    crate::view::get_full_year::get_full_year()
}

#[actix_web::put("/install")]
async fn install(
    pool: actix_web::web::Data<mysql::Pool>,
    data: actix_web::web::Json<crate::view::install_view::Data>
) -> impl actix_web::Responder {
    crate::view::install_view::install(pool.into_inner(), data.into_inner())
}

#[actix_web::get("/already-installed")]
async fn already_installed(
    pool: actix_web::web::Data<mysql::Pool>
) -> impl actix_web::Responder {
    crate::view::already_installed_view::already_installed(pool.into_inner())
}

#[actix_web::main]
pub async fn fire() -> std::io::Result<()> {
    // Logger initialization
    env_logger::init();

    // SeptBlog's config
    let config_file = match std::fs::read_to_string("/etc/septblog/backend.json") {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to read configuration file.");
            log::debug!("{:?}", err);
            panic!();
        }
    };
    let config = match json::parse(&config_file) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to parse configuration file.");
            log::debug!("{:?}", err);
            panic!();
        }
    };

    // Create database pool
    let pool = crate::db::database::database_pool(
        &config["database"]["host"].to_string(),
        config["database"]["port"].as_u16().unwrap(),
        &config["database"]["username"].to_string(),
        &config["database"]["password"].to_string(),
        &config["database"]["name"].to_string()
    );

    let allowed_origin = format!("{}{}:{}", &config["frontend"]["protocol"], &config["frontend"]["host"], &config["frontend"]["port"]);

    let host = config["host"].clone().to_string();
    let port = config["port"].clone().as_u16().unwrap();

    /*
    // Create image upload directory
    std::fs::create_dir_all("./uploads").unwrap();
     */

    actix_web::HttpServer::new( move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_header(actix_web::http::header::AUTHORIZATION)
            .allowed_header(actix_web::http::header::ACCEPT)
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .allowed_header(actix_web::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
            .supports_credentials()
            .max_age(3600);

        actix_web::App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(config.clone()))
            .wrap(cors)

            .service(admin_check_credentials)
            .service(admin_signup)
            .service(admin_login)
            .service(admin_article_editor)
            .service(admin_article_editor_get_article_categories)
            .service(admin_article_editor_get_article)
            .service(admin_dashboard_chart)
            .service(admin_dashboard_visitors)
            .service(admin_visitors)
            .service(admin_articles)
            .service(admin_articles_move_to_trash)
            .service(admin_trashed_articles)
            .service(admin_articles_delete)
            .service(admin_profile_get)
            .service(admin_profile_put)
            .service(admin_users_get)
            .service(admin_users_put)
            .service(admin_settings_get)
            .service(admin_settings_put)
            .service(admin_security_change_password)
            .service(admin_security_change_email)
            .service(admin_logged_in_as)
            .service(admin_get_user_level)

            .service(articles)
            .service(article)
            .service(category)
            .service(user)
            .service(get_site_title)
            .service(get_site_tagline)
            .service(get_full_year)
            .service(install)
            .service(already_installed)

            .service(admin_image_upload)
            //.service(actix_files::Files::new("/uploads", "./uploads").show_files_listing())
            .service(actix_files::Files::new("/uploads", "/var/www/septblog/uploads"))
    })
    .bind((host, port))?
    .run()
    .await
}