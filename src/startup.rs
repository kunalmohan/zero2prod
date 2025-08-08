use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::*;

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
