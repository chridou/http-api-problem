mod serialization {
    use crate::HttpApiProblem;
    use http::StatusCode;
    use serde_json::{self, json};

    #[test]
    fn should_serialize_status_present_correctly() {
        let prob = HttpApiProblem::with_title(StatusCode::NOT_FOUND);

        let sample = serde_json::to_value(prob).unwrap();
        let expected = json!({
            "title": "Not Found",
            "status": 404
        });

        assert_eq!(sample, expected);
    }

    #[test]
    fn should_serialize_status_apsent_correctly() {
        let prob = HttpApiProblem::empty().title("foo");

        let sample = serde_json::to_value(prob).unwrap();
        let expected = json!({
            "title": "foo"
        });

        assert_eq!(sample, expected);
    }

    #[test]
    fn deserialize_status_present() {
        let json = r#"{"title": "foo", "status": 500}"#;

        let prob: HttpApiProblem = serde_json::from_str(json).unwrap();

        assert_eq!(prob.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[test]
    fn deserialize_status_apsent() {
        let json = r#"{"title": "foo"}"#;

        let prob: HttpApiProblem = serde_json::from_str(json).unwrap();

        assert_eq!(prob.status, None);
    }

    #[test]
    fn deserialize_status_null() {
        let json = r#"{"title": "foo", "status": null}"#;

        let prob: HttpApiProblem = serde_json::from_str(json).unwrap();

        assert_eq!(prob.status, None);
    }
}
