use actix_web::{App, get, HttpServer, Responder, web};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world")
}


#[get("/{name}/greet")]
async fn name_index(web::Path((name, )): web::Path<(String, )>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(name_index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
