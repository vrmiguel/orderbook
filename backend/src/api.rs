use std::borrow::Cow;

use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};

use crate::SharedOrderRepositoryImpl;

pub mod forms;
mod routes;

pub fn spawn_server(
    storage: SharedOrderRepositoryImpl,
    (address, port): (Cow<'static, str>, u16),
) -> crate::Result<Server> {
    let server = HttpServer::new(move || {
        // Due to `Fn` move semantics silliness we have to re-clone
        // this within the closure. This is not a problem since we're just
        // cloning an Arc.
        let storage = storage.clone();
        let endpoints = web::scope("/orders")
            .service(routes::create_ask)
            .service(routes::create_bid)
            .service(routes::list_all)
            .service(routes::cancel_order)
            .app_data(Data::new(storage));

        if cfg!(debug_assertions) {
            // Use permissive CORS headers for dev environments.
            // Useful, for example, to test the frontend service locally.
            App::new()
                .wrap(actix_cors::Cors::permissive())
                .service(endpoints)
        } else {
            // Use restrictive CORS headers for production environments
            App::new()
                .wrap(actix_cors::Cors::default())
                .service(endpoints)
        }
    })
    .workers(12)
    .max_connections(50_000)
    .bind((&*address, port))?
    .run();

    tracing::info!("Server spawned!");

    Ok(server)
}
