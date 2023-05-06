mod chat_gpt;
mod request;

use crate::chat_gpt::{chat_gpt_wrapper, ChatGPTMessage, ChatGPTMessageHandler, Role};
use std::env;
use std::error::Error;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use reqwest::Client;

use actix_web::web::Data;

use actix_web::middleware::Logger;
use std::sync::Arc;

async fn generate(
    req: HttpRequest,
    system_message: Data<ChatGPTMessage>,
    chat_prompt: Data<Arc<Box<ChatGPTMessageHandler>>>,
) -> impl Responder {
    let path = req.path().to_string();

    let messages = vec![
        system_message.get_ref().clone(),
        ChatGPTMessage {
            role: Role::Assistant,
            content: path,
        },
    ];

    let result: Result<ChatGPTMessage, Box<dyn Error>> = chat_prompt(messages).await;

    match result {
        Ok(response) => HttpResponse::Ok()
            .content_type("text/html")
            .body(response.content),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static! {
        static ref CLIENT: Client = Client::new();
    }
    let client = &*CLIENT;
    let chat_prompt: Arc<Box<ChatGPTMessageHandler>> = Arc::new(chat_gpt_wrapper(client));

    let system_message = ChatGPTMessage {
        role: Role::System,
        content: r#"You are an AI assistant programmed to generate HTML pages based on the URL path provided to you.
You must unleash your creativity and generate wildly descriptive content, akin to what one might find on Wikipedia.
Your task is to include headers such as h1, h2, etc., and to format the HTML page properly in general.
Additionally, you must include any relevant information, even if it appears unrelated or seemingly random.
If the URL path provided to you is "/", you should generate a random page about any topic.
This will allow you to showcase your ability to generate imaginative content on a wide range of subjects.
The aim is to be as imaginative and inventive as possible, while avoiding the inclusion of any image tags.
Furthermore, your client has asked that you include a specific CSS stylesheet in the generated HTML pages.
Please ensure that the following CSS link is included in the head section of each HTML page:

<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css\">
This is a must-have requirement and should not be omitted.
Additionally, your client requires that the body content of each HTML page be wrapped 
inside a main container element with the class 'container'. 
This will help to ensure consistent formatting and styling across all pages. 
Please make sure to include this container element in every HTML page you generate."#.to_string(),
    };

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(chat_prompt.clone()))
            .app_data(Data::new(system_message.clone()))
            .route("/{tail:.*}", web::get().to(generate))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
