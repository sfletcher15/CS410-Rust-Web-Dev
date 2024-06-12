use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(Vec::new);

    {
        let questions = questions.clone();
        use_effect_with_deps(
            move |_| {
                let questions = questions.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get("http://127.0.0.1:3030/questions").send().await {
                        Ok(response) => {
                            web_sys::console::log_1(&"Received response".into());
                            if response.ok() {
                                let json_response = response.text().await.unwrap();
                                web_sys::console::log_1(&format!("Raw JSON: {}", json_response).into());
                                match serde_json::from_str::<QuestionsResponse>(&json_response) {
                                    Ok(fetched_questions) => {
                                        web_sys::console::log_1(&"Questions fetched successfully".into());
                                        web_sys::console::log_1(&format!("{:?}", fetched_questions.questions).into());
                                        questions.set(fetched_questions.questions);
                                    }
                                    Err(err) => {
                                        web_sys::console::log_1(&"Failed to parse JSON".into());
                                        web_sys::console::log_1(&format!("{:?}", err).into());
                                    }
                                }
                            } else {
                                web_sys::console::log_1(&"Request failed".into());
                                web_sys::console::log_1(&format!("Status: {}", response.status()).into());
                            }
                        }
                        Err(err) => {
                            web_sys::console::log_1(&"Failed to fetch questions".into());
                            web_sys::console::log_1(&format!("{:?}", err).into());
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            <h1>{ "Questions" }</h1>
            <ul>
                { for questions.iter().map(|q| html! { <li>{ format!("{} - {}", q.question_text, q.id) }</li> }) }
            </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties, Deserialize)]
struct QuestionRecord {
    id: i32,
    question_text: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct QuestionsResponse {
    questions: Vec<QuestionRecord>,
}

fn main() {
    yew::start_app::<App>();
}
