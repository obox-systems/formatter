use regex::Regex;

pub struct Bracket;

impl crate::core::Plugin for Bracket {
    fn positive() -> Vec<&'static str> {
        vec![r"\s*\[\s*[^]]*\]\s*"]
    }

    fn run(slice: &str) -> String {
        let trimmed = slice.trim();
        if trimmed == "[]" {
            return trimmed.to_owned();
        }

        let re = Regex::new(r"\s*\[\s*([^]]*)\]\s*").unwrap();
        if let Some(caps) = re.captures(trimmed) {
            if let Some(inner) = caps.get(1) {
                let inner_trimmed = inner.as_str().trim();
                if inner_trimmed.is_empty() {
                    return "[ ]".to_owned();
                } else {
                    // Split the inner content by comma, trim each element, then join them back with comma and space
                    let formatted_inner: String = inner_trimmed
                        .split(',')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>()
                        .join(", ");
                    return format!("[ {} ]", formatted_inner);
                }
            }
        }

        trimmed.to_owned()
    }
}
