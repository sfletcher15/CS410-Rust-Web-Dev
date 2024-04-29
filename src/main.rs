use axum::{
    extract::Path,
    routing::{get, post, put, delete},
    Router,
    Json,
    response::IntoResponse,
};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

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

async fn get_questions() -> impl IntoResponse {
    println!("Fetching all questions");
    let questions = QUESTIONS.lock().unwrap().clone();
    println!("Current questions: {:?}", questions);
    Json(json!({ "questions": questions }))
}

async fn post_question(Json(data): Json<Question>) -> impl IntoResponse {
    println!("Adding new question");
    let mut questions = QUESTIONS.lock().unwrap();
    let next_id = questions.len() as i32 + 1;
    questions.insert(next_id, data.question.clone());
    println!("Added: {} as ID {}", data.question, next_id);
    Json(json!({ "message": "Question added successfully", "question_id": next_id}))
}

async fn update_question(Path(id): Path<i32>, Json(data): Json<Question>) -> impl IntoResponse {
    println!("Updating question ID {}", id);
    let mut questions = QUESTIONS.lock().unwrap();
    if questions.contains_key(&id) {
        questions.insert(id, data.question.clone());
        println!("Updated ID {} with new question {}", id, data.question);
        Json(json!({ "message": "Question updated successfully", "question_id": id }))
    } else {
        println!("No question found with ID {}", id);
        Json(json!({ "error": "Question not found", "question_id": id }))
    }
}

async fn delete_question(Path(id): Path<i32>) -> impl IntoResponse {
    println!("Deleting question ID {}", id);
    let mut questions = QUESTIONS.lock().unwrap();
    if questions.remove(&id).is_some() {
        println!("Deleted question ID {}", id);
        Json(json!({ "message": "Question deleted successfully", "question_id": id }))
    } else {
        println!("No question found with ID {}", id);
        Json(json!({ "error": "Question not found", "question_id": id }))
    }
}

#[tokio::main]
async fn main() {
    // Setup the router
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(post_question))
        .route("/questions/:id", put(update_question))
        .route("/questions/:id", delete(delete_question));

    // Define the address and run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
