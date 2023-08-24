pub struct Operators;

impl crate::core::Plugin for Operators {
    fn positive() -> Vec<&'static str> {
        std::iter::empty()
            .chain(super::Braces::negative())
            .chain(super::Bracket::negative())
            .chain(super::Ident::negative())
            .chain(super::Parentheses::negative())
            .collect()
    }

    fn run(slice: &str) -> String {
        dbg!(slice);
        format!(" {slice} ")
    }
}
