pub struct Operators;

impl crate::core::Plugin for Operators {
    fn positive() -> Vec<&'static str> {
        vec![r"\(\s*", r"\s*\)", r"\s*,\s*"]
    }

    fn run(slice: &str) -> String {
        format!(" {} ", slice.trim())
    }
}
