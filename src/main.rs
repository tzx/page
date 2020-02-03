use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};

fn css(cfg: &mut web::ServiceConfig) {
    cfg.service(
	    actix_files::Files::new("/css", "./css").show_files_listing()
    );
}

fn static_files(cfg: &mut web::ServiceConfig) {
    cfg.service(
        actix_files::Files::new("/static", "./static").show_files_listing()
    );
}

async fn index1() -> Result<NamedFile> {
    let path = "./templates/index.html";
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
	    .configure(css)
        .configure(static_files)
        .route("/", web::get().to(index1))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

