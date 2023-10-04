pub struct Braces;

impl crate::core::Plugin for Braces {
    fn positive() -> Vec<&'static str> {
        vec!["\\{\\s*\\}", "\\{\\s*", "\\}\\s*"]
    }

    fn run(slice: &str) -> String {
        let trimmed = slice.trim();
        if trimmed == "{" {
            return "{ ".to_owned();
        } else if trimmed == "}" {
            return " }".to_owned();
        }
        // If the string does not match any defined format, return it as is.
        trimmed.to_owned()
    }
}
