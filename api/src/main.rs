use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use crate::{request_input::CreateWebsteInput, request_output::CreateWebsteOutput};
use store::Store;
pub mod request_input;
pub mod request_output;

// get endpoint for getting the website
#[handler]
// path is used for params (/:sdfjladsfj)
fn get_website(Path(name): Path<String>) -> String {
    let s = Store{};
    s.create_user();
    format!("hello: {name}")
}


// Post Endpoint for creating website
// In TypeScript, this would be like:
// function createWebsite(data: { url: string }): { id: string }
// automatically parse JSON request bodies into JavaScript objects.
// It handles deserialization of the incoming JSON into our CreateWebsteInput struct
// and serialization of our CreateWebsteOutput struct back into JSON.
#[handler]
fn create_website(Json(data): Json<CreateWebsteInput>) -> Json<CreateWebsteOutput> {
    let _url = data.url;
    
    let s = Store{};
    let id = s.create_website();
    // persist it in db 
    // sqlx => postgreqs with raw sql
    // diesel => prisma (orm)
    let resposne =  CreateWebsteOutput{
        id
    };
    Json(resposne)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
            .at("/website/:name", get(get_website))
            .at("/website/", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("better-uptime")
        .run(app)
        .await
}