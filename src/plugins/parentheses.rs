pub struct Parentheses;

impl crate::core::Plugin for Parentheses {
    fn positive() -> Vec<&'static str> {
        vec![r"\(|\)"]
    }

    fn negative() -> Vec<&'static str> {
        vec![r"[^\(|\)]"]
    }

    fn run(slice: &str) -> String {
        // format!(" {slice} ")
        slice.to_string()
    }
}
