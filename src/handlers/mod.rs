use rocket::{serde::json::Json};

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    QuestionDetail {
        question_uuid: "[UUID of a created question]".to_string(),
        title: question.title,
        description: question.description,
        created_at: "2021-08-25T00:00:00Z".to_string(),
    }
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    vec![QuestionDetail {
        question_uuid: "[UUID of a created question]".to_string(),
        title: "How do I create a REST API?".to_string(),
        description: "I want to create a REST API using Rust and Rocket.".to_string(),
        created_at: "2021-08-25T00:00:00Z".to_string(),
    }]
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    QuestionId {
        question_uuid: question_uuid.question_uuid,
    }
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer (
    answer: Json<Answer>,
) -> Json<AnswerDetail> {
    AnswerDetail {
        answer_uuid: "[UUID of a created answer]".to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "2021-08-25T00:00:00Z".to_string(),
    }
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    vec![AnswerDetail {
        answer_uuid: "[UUID of a created answer]".to_string(),
        question_uuid: "[UUID of a created question]".to_string(),
        content: "You should use Rocket!".to_string(),
        created_at: "2021-08-25T00:00:00Z".to_string(),
    }]
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//       
//       hint: this function should look very similar to the delete_question function above

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>
) {
    AnswerId {
        answer_uuid: answer_uuid.answer_uuid,
    }
}