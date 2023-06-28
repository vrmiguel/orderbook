use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};

use crate::storage::OrderStorage;

pub mod forms;
mod routes;

pub fn spawn_server(storage: OrderStorage) -> crate::Result<Server> {
    let server = HttpServer::new(move || {
        // Due to `Fn` move semantics silliness we have to re-clone
        // this within the closure. This is not a problem since we're just
        // cloning an Arc.
        let storage = storage.clone();
        App::new().service(
            web::scope("/orders")
                .service(routes::create_ask)
                .service(routes::create_bid)
                .service(routes::list_all)
                .app_data(Data::new(storage)),
        )
    })
    .workers(12)
    .max_connections(50_000)
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
