// use std::net::TcpListener; -> you can create a http listener on top of this

use poem::{Route, Server, get, handler, listener::TcpListener, post, web::Path};

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

#[handler]
fn create_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // specify the business details of your app
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/status/:website_id", get(create_website))
        .at("/website", post(create_website));
    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
