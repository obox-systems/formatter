pub struct Bracket;

impl crate::core::Plugin for Bracket {
    fn positive() -> &'static [&'static str] {
        &[r"\[|\]"]
    }

    fn run(slice: &str) -> String {
        format!(" {slice} ")
    }
}
