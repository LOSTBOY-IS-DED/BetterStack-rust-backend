// use std::net::TcpListener; -> you can create a http listener on top of this

use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Path};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
