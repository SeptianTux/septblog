use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mime_guess::Mime;

fn get(path: String) -> impl Responder {
    match std::fs::read(&path) {
        Ok(val) => {
            let mime = get_mime_type(path);
            return HttpResponse::Ok().content_type(mime).body(val)
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                return HttpResponse::NotFound().body("Not found.")
            } else {
                return HttpResponse::InternalServerError().body("Internal server error.")
            }
        }
    }
}

fn get_mime_type(file_path: String) -> Mime {
    // Detect mime type by file extension
    let mime_type = mime_guess::from_path(file_path).first_or(mime_guess::mime::APPLICATION_OCTET_STREAM);
    let str = mime_type;

    str
}

#[get("/")]
async fn index() -> impl Responder {
    let file = "./public/index.html".to_string();

    return get(file)
}

#[get("/404")]
async fn not_found() -> impl Responder {
    let file = "./public/404.html".to_string();

    return get(file)
}

#[get("/about")]
async fn about() -> impl Responder {
    let file = "./public/about.html".to_string();

    return get(file)
}

#[get("/article/{name}")]
async fn article(_name: web::Path<String>) -> impl Responder {
    let file = "./public/article.html".to_string();

    return get(file)
}

#[get("/category/{name}")]
async fn category(_path: web::Path<String>) -> impl Responder {
    let file = "./public/category.html".to_string();

    return get(file)
}

#[get("/contact")]
async fn contact() -> impl Responder {
    let file = "./public/contact.html".to_string();

    return get(file)
}

#[get("/user/{name}")]
async fn user(_name: web::Path<String>) -> impl Responder {
    let file = "./public/user.html".to_string();

    return get(file)
}

#[get("/install")]
async fn install() -> impl Responder {
    HttpResponse::Found().append_header(("Location", "/install/stage/1")).finish()
}

#[get("/install/stage/1")]
async fn install_stage_1() -> impl Responder {
    let file = "./public/install-stage-1.html".to_string();

    return get(file)
}

#[get("/install/stage/2")]
async fn install_stage_2() -> impl Responder {
    let file = "./public/install-stage-2.html".to_string();

    return get(file)
}

#[get("/install/finish")]
async fn install_finish() -> impl Responder {
    let file = "./public/install-finish.html".to_string();

    return get(file)
}

#[get("/already-installed")]
async fn already_installed() -> impl Responder {
    let file = "./public/already-installed.html".to_string();

    return get(file)
}

#[get("/css/{name}")]
async fn css(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/css/".to_owned() + &name;

    return get(file)
}

#[get("/js/{name}")]
async fn js(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/js/".to_owned() + &name;

    return get(file)
}

#[get("/js/View/{name}")]
async fn js_view(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/js/View/".to_owned() + &name;

    return get(file)
}

#[get("/js/ViewModel/{name}")]
async fn js_viewmodel(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/js/ViewModel/".to_owned() + &name;

    return get(file)
}

#[get("/js/Model/{name}")]
async fn js_model(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/js/Model/".to_owned() + &name;

    return get(file)
}

#[get("/assets/img/{name}")]
async fn assets_img(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/assets/img/".to_owned() + &name;

    return get(file)
}

#[get("/admin")]
async fn admin() -> impl Responder {
    let file: String = "./public/admin/index.html".to_string();

    return get(file)
}

#[get("/admin/{name}")]
async fn admin_path(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/".to_owned() + &name + ".html";

    return get(file)
}

#[get("/admin/article-editor/{name}")]
async fn admin_article_editor_id() -> impl Responder {
    let file: String = "./public/admin/article-editor.html".to_string();

    return get(file)
}

#[get("/admin/security/{name}")]
async fn admin_security(name: web::Path<String>) -> impl Responder {
    let mut file = String::new();
    let path = name.into_inner();

    if path == "change-password" {
        file.push_str("./public/admin/change-password.html");
    } else if path == "change-email-address" {
        file.push_str("./public/admin/change-email-address.html");
    }
    
    return get(file)
}

#[get("/admin/css/{name}")]
async fn admin_css(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/css/".to_owned() + &name;

    return get(file)
}

#[get("/admin/js/{name}")]
async fn admin_js(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/js/".to_owned() + &name;

    return get(file)
}

#[get("/admin/js/View/{name}")]
async fn admin_js_view(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/js/View/".to_owned() + &name;

    return get(file)
}

#[get("/admin/js/ViewModel/{name}")]
async fn admin_js_viewmodel(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/js/ViewModel/".to_owned() + &name;

    return get(file)
}

#[get("/admin/js/Model/{name}")]
async fn admin_js_model(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/js/Model/".to_owned() + &name;

    return get(file)
}

#[get("/admin/js/Errors/{name}")]
async fn admin_js_errors(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/js/Errors/".to_owned() + &name;

    return get(file)
}

#[get("/admin/js/Utils/{name}")]
async fn admin_js_utils(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/js/Utils/".to_owned() + &name;

    return get(file)
}

#[get("/admin/chartjs@4.5.0/{name}")]
async fn admin_chartjs(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/chartjs@4.5.0/".to_owned() + &name;

    return get(file)
}

#[get("/admin/fontawesome@6.3.0/{name}")]
async fn admin_fontawesome(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/fontawesome@6.3.0/".to_owned() + &name;

    return get(file)
}

#[get("/admin/quill@2.0.3/{name}")]
async fn admin_quill(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/quill@2.0.3/".to_owned() + &name;

    return get(file)
}

#[get("/admin/bootstrap@5.2.3/{name}")]
async fn admin_bootstrap(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/bootstrap@5.2.3/".to_owned() + &name;

    return get(file)
}

#[get("/admin/tagify/{name}")]
async fn admin_tagify(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/tagify/".to_owned() + &name;

    return get(file)
}

#[get("/admin/assets/img/{name}")]
async fn admin_assets_img(name: web::Path<String>) -> impl Responder {
    let file: String = "./public/admin/assets/img/".to_owned() + &name;

    return get(file)
}

async fn proxy_uploads(
    config: actix_web::web::Data<json::JsonValue>,
    req: actix_web::HttpRequest
) -> HttpResponse {
    let path = req.match_info().query("tail");
    let backend_url = format!(
        "{}{}:{}/uploads/{}",
        config["backend"]["protocol"].to_string(),
        config["backend"]["host"].to_string(),
        config["backend"]["port"].to_string(), path
    );

    let client = awc::Client::default();
    let backend_resp = client.get(backend_url).send().await;

    match backend_resp {
        Ok(mut resp) => {
            let mut builder = HttpResponse::build(resp.status());

            for (header_name, header_value) in resp.headers() {
                builder.insert_header((header_name.clone(), header_value.clone()));
            }

            let body = resp.body().await.unwrap_or_else(|_| web::Bytes::from_static(b""));

            builder.body(body)
        }
        Err(_) => HttpResponse::BadGateway().body("Failed to reach backend"),
    }
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Logger initialization
    env_logger::init();

    // SeptBlog's config
    let config_file = match std::fs::read_to_string("/etc/septblog/frontend.json") {
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

    let host = config["host"].clone();
    let port = config["port"].clone();

    let host_str = host.as_str().unwrap();
    let port_u16 = port.as_u16().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(index)
            .service(not_found)
            .service(about)
            .service(article)
            .service(category)
            .service(contact)
            .service(user)
            .service(install)
            .service(install_stage_1)
            .service(install_stage_2)
            .service(install_finish)
            .service(already_installed)

            .service(css)
            .service(js)
            .service(js_view)
            .service(js_viewmodel)
            .service(js_model)
            .service(assets_img)
            .service(admin)

            .service(admin_path)
            .service(admin_security)
            .service(admin_article_editor_id)

            .service(admin_css)
            .service(admin_js)
            .service(admin_chartjs)
            .service(admin_tagify)
            .service(admin_quill)
            .service(admin_fontawesome)
            .service(admin_bootstrap)
            .service(admin_assets_img)
            .service(admin_js_view)
            .service(admin_js_viewmodel)
            .service(admin_js_model)
            .service(admin_js_errors)
            .service(admin_js_utils)

            .route("/uploads/{tail:.*}", web::to(proxy_uploads))
    })
    .bind((host_str, port_u16))?
    .run()
    .await
}
