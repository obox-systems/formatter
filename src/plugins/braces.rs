pub struct Braces;

impl crate::core::Plugin for Braces {
    fn positive() -> Vec<&'static str> {
        vec![r"\{"]
    }

    fn negative() -> Vec<&'static str> {
        vec![r"[^\{]"]
    }

    fn run(slice: &str) -> String {
        format!("\n{slice}\n{{")
    }
}
