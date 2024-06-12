use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::net::SocketAddr;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Serialize)]
struct Question {
    question: String,
}

#[derive(Deserialize, Serialize)]
struct Answer {
    answer: String,
}

#[derive(Deserialize, Serialize)]
struct QuestionWithAnswers {
    id: i32,
    question: String,
    answers: Vec<String>,
}

#[derive(Serialize)]
struct QuestionRecord {
    id: i32,
    question_text: String,
}

async fn get_questions(Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let questions = sqlx::query_as!(QuestionRecord, "SELECT id, question_text FROM questions")
        .fetch_all(&pool)
        .await;

    match questions {
        Ok(qs) => Json(json!({ "questions": qs })),
        Err(e) => {
            eprintln!("Failed to fetch questions: {}", e);
            Json(json!({ "error": "Failed to fetch questions" }))
        }
    }
}

async fn post_question(Json(data): Json<Question>, Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO questions (question_text) VALUES ($1) RETURNING id",
        data.question
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(res) => Json(json!({ "message": "Question added successfully", "question_id": res.id })),
        Err(e) => {
            eprintln!("Failed to add question: {}", e);
            Json(json!({ "error": "Failed to add question" }))
        }
    }
}

async fn update_question(Path(id): Path<i32>, Json(data): Json<Question>, Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE questions SET question_text = $1, updated_at = NOW() WHERE id = $2",
        data.question, id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                Json(json!({ "error": "Question not found", "question_id": id }))
            } else {
                Json(json!({ "message": "Question updated successfully", "question_id": id }))
            }
        }
        Err(e) => {
            eprintln!("Failed to update question: {}", e);
            Json(json!({ "error": "Failed to update question" }))
        }
    }
}

async fn delete_question(Path(id): Path<i32>, Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM questions WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                Json(json!({ "error": "Question not found", "question_id": id }))
            } else {
                Json(json!({ "message": "Question deleted successfully", "question_id": id }))
            }
        }
        Err(e) => {
            eprintln!("Failed to delete question: {}", e);
            Json(json!({ "error": "Failed to delete question" }))
        }
    }
}

async fn add_answer(Path(question_id): Path<i32>, Json(data): Json<Answer>, Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO answers (question_id, answer_text) VALUES ($1, $2) RETURNING id",
        question_id, data.answer
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(res) => Json(json!({ "message": "Answer added successfully", "answer_id": res.id })),
        Err(e) => {
            eprintln!("Failed to add answer: {}", e);
            Json(json!({ "error": "Failed to add answer" }))
        }
    }
}

async fn get_question_with_answers(Path(id): Path<i32>, Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let question = sqlx::query!("SELECT id, question_text FROM questions WHERE id = $1", id)
        .fetch_one(&pool)
        .await;

    match question {
        Ok(q) => {
            let answers = sqlx::query!("SELECT answer_text FROM answers WHERE question_id = $1", id)
                .fetch_all(&pool)
                .await;

            match answers {
                Ok(ans) => {
                    let answers_text = ans.into_iter().map(|record| record.answer_text).collect::<Vec<String>>();
                    Json(json!({
                        "id": q.id,
                        "question": q.question_text,
                        "answers": answers_text,
                    }))
                }
                Err(e) => {
                    eprintln!("Failed to fetch answers: {}", e);
                    Json(json!({ "error": "Failed to fetch answers" }))
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch question: {}", e);
            Json(json!({ "error": "Failed to fetch question" }))
        }
    }
}

async fn initialize_database(pool: &sqlx::PgPool) {
    if let Err(e) = pool.execute(
        "
        CREATE TABLE IF NOT EXISTS questions (
            id SERIAL PRIMARY KEY,
            question_text TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS answers (
            id SERIAL PRIMARY KEY,
            question_id INTEGER NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
            answer_text TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "
    ).await {
        eprintln!("Failed to create tables: {}", e);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database URL: {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Initialize the database
    initialize_database(&pool).await;

    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(post_question))
        .route("/questions/:id", put(update_question))
        .route("/questions/:id", delete(delete_question))
        .route("/questions/:id/answers", post(add_answer))
        .route("/questions/:id", get(get_question_with_answers))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
