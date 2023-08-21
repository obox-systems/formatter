pub struct Spaces;

impl crate::core::Plugin for Spaces {
    regex!(r"\(|\)", r"\[|\]", r"[-+*/%^&|<>=]");

    fn run(slice: &str) -> String {
        format!(" {slice} ")
    }
}
