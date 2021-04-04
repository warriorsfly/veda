use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    act::{add_redis, add_websocket},
    handler::socket_route,
};

pub async fn serv() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_websocket)
            .configure(add_redis)
            .service(web::resource("/notify/").to(socket_route))
        // static resources
        // .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
