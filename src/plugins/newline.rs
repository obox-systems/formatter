pub struct Newline;

impl crate::core::Plugin for Newline {
    regex!(r"\{");

    fn run(slice: &str) -> String {
        format!("\n{slice}\n{{")
    }
}
