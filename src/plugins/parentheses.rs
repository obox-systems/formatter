use regex::Regex;

pub struct Parentheses;

impl crate::core::Plugin for Parentheses {
    fn positive() -> Vec<&'static str> {
        vec![r"\s*\(\s*[^)]*\)\s*"]
    }

    fn run(slice: &str) -> String {
        let trimmed = slice.trim();
        if trimmed == "()" {
            return trimmed.to_owned();
        }

        let re = Regex::new(r"\s*\(\s*([^)]*)\)\s*").unwrap();
        if let Some(caps) = re.captures(trimmed) {
            if let Some(inner) = caps.get(1) {
                let inner_trimmed = inner.as_str().trim();
                if inner_trimmed.is_empty() {
                    return "( )".to_owned();
                } else {
                    return format!("( {} )", inner_trimmed);
                }
            }
        }

        trimmed.to_owned()
    }
}
