extern crate http_api_problem;
#[macro_use]
extern crate serde;
extern crate serde_json;

use http_api_problem::*;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut problem = HttpApiProblem::with_title_and_type_from_status(StatusCode::INTERNAL_SERVER_ERROR);

    problem.set_value("error", &"this sucks".to_string()).unwrap();
    problem.set_value("everything", &42).unwrap();
    problem
        .set_value(
            "person",
            &Person {
                name: "Peter".into(),
                age: 77,
            },
        )
        .unwrap();

    let json = problem.json_string();

    println!("{}", json);

    let parsed: HttpApiProblem = serde_json::from_str(&json).unwrap();

    println!("\n\n{:#?}", parsed);
}
