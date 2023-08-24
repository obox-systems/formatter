pub struct Bracket;

impl crate::core::Plugin for Bracket {
    fn positive() -> Vec<&'static str> {
        vec![r"\[|\]"]
    }

    fn negative() -> Vec<&'static str> {
        vec![r"[^\[|\]]"]
    }

    fn run(slice: &str) -> String {
        format!(" {slice} ")
    }
}
