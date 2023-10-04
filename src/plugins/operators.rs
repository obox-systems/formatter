pub struct Operators;

impl crate::core::Plugin for Operators {
    fn positive() -> Vec<&'static str> {
        vec![
            // This pattern uses look-behind and look-ahead assertions to ensure that
            // there are no spaces before or after the matched character,
            // which is assumed to be an operator.
            r"(?<![^\w\s\[\](){}])\s*[^\w\s\[\](){}]\s*(?![^\w\s\[\](){}])",
        ]
    }

    fn run(slice: &str) -> String {
        // The regex ensures there are no spaces before or after the operator,
        // so we just need to trim any spaces that are part of the match itself.
        format!(" {} ", slice.trim())
    }
}
