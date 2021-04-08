use serde::{Deserialize, Serialize};

use http_api_problem::*;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let problem = HttpApiProblem::with_title_and_type(StatusCode::INTERNAL_SERVER_ERROR)
        .value("error", &"this sucks")
        .value("everything", &42)
        .value(
            "person",
            &Person {
                name: "Peter".into(),
                age: 77,
            },
        );

    let json = problem.json_string();

    println!("{}", json);

    let parsed: HttpApiProblem = serde_json::from_str(&json).unwrap();

    println!("\n\n{:#?}", parsed);
}
