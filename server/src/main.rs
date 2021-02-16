use actix_web::{App, error, Error, get, HttpResponse, HttpServer, web};
use tera::{Context, Tera};

use actix_files as fs;

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let ctx = Context::new();
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let views = Tera::new("view/**/*").unwrap();

    HttpServer::new(move ||
        {
            App::new()
                .service(index)
                .service(fs::Files::new("/static", "static").show_files_listing())
                .data(views.clone())
        }).bind("0.0.0.0:8080")?
        .run()
        .await
}
