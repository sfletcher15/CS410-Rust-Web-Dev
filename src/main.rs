use axum::{
    routing::{get, post},
    Router,
    Json,
    response::IntoResponse,
};
use std::{
    collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}
};
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Question {
    question: String,
}

// Create a global hash map to store questions
type Questions = Arc<Mutex<HashMap<i32, String>>>;

// Initialize the global Questions HashMap
lazy_static::lazy_static! {
    static ref QUESTIONS: Questions = Arc::new(Mutex::new({
        let mut map = HashMap::new();
        map.insert(1, "What is Rust?".to_string());
        map.insert(2, "How does memory safety work in Rust?".to_string());
        map
    }));
}

// Define the handler function
async fn get_questions() -> impl IntoResponse {
    let questions = QUESTIONS.lock().unwrap().clone();
    Json(json!({ "questions": questions }))
}

async fn post_question(Json(data): Json<Question>) -> impl IntoResponse {
    let mut questions = QUESTIONS.lock().unwrap();
    let next_id = questions.len() as i32 + 1;
    questions.insert(next_id, data.question.clone());

    Json(json!({ "message": "Question added successfully", "question_id": next_id}))
}

#[tokio::main]
async fn main() {
    // Setup the router
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(post_question));

    // Define the address and run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}