use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse};

fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
	fs::Files::new("/", "./templates").index_file("index.html")
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
	    .configure(index)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

