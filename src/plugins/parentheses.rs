pub struct Parentheses;

impl crate::core::Plugin for Parentheses {
    fn positive() -> &'static [&'static str] {
        &[r"\(|\)", r"[-+*/%^&|<>=]"]
    }

    fn run(slice: &str) -> String {
        format!(" {slice} ")
    }
}
