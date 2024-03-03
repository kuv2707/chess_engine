use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::{io, sync::Mutex};
mod models;
mod controllers;
mod engine;
mod stockfish_adapter;
use stockfish_adapter::stockfish_adapter as stockfish;
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/bestmove").route(web::post().to(controllers::bestmove)))
        .service(web::resource("/makemove").route(web::post().to(controllers::makemove)))
        .service(
            web::resource("/piecewisemoves").route(web::post().to(controllers::piecewisemoves)),
        )
        .service(
            web::resource("/stockfishbestmove")
                .route(web::post().to(controllers::stockfishbestmove)),
        )
        .service(
            web::resource("/stockfishmakemove").route(web::post().to(controllers::stockfishmove)),
        );
}

pub struct AppState {
    pub stockfish: Mutex<stockfish>,
}

#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    let mut stockfish = stockfish::new();
    stockfish.newgame();
    let data = web::Data::new(AppState {
        stockfish: Mutex::new(stockfish),
    });

    let server = HttpServer::new(move || {
        App::new().app_data(data.clone()).configure(config).wrap(
            Cors::permissive()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header(),
        )
    })
    .bind("localhost:4000");

    println!("Server is ready");
    server?.run().await
}
