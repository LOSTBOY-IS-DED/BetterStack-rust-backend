// use std::net::TcpListener; -> you can create a http listener on top of this

use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};

use crate::{request_inputs::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use store::Store;
pub mod request_inputs;
pub mod request_output;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

// getting json input and receiving json input
#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    // persist this is a db
    // calling and using the store
    let s = Store {};
    let id = s.create_website();
    let response = CreateWebsiteOutput { id };
    Json(response)
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
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
