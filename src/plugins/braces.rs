pub struct Braces;

impl crate::core::Plugin for Braces {
    fn positive() -> &'static [&'static str] {
        &[r"\{"]
    }

    fn run(slice: &str) -> String {
        format!("\n{slice}\n{{")
    }
}
