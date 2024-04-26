use serde::Serialize;
use std::str::FromStr;
use std::io::{Error, ErrorKind};
use axum::{
    routing::{get},
    Router,
    Json,
    response::IntoResponse
};
use std::net::SocketAddr;

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        if id.is_empty() {
            Err(Error::new(ErrorKind::InvalidInput, "No ID provided"))
        } else {
            Ok(QuestionId(id.to_string()))
        }
    }
}

// Define the handler function
async fn get_questions() -> impl IntoResponse {
    let question = Question::new(
        QuestionId::from_str("1").expect("No ID provided"),
        "First Question".to_string(),
        "Content of the question".to_string(),
        Some(vec!["faq".to_string()])
    );

    Json(question)
}

#[tokio::main]
async fn main() {
    // Setup the router
    let app = Router::new()
        .route("/questions", get(get_questions));

    // Define the address and run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}