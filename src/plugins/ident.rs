pub struct Ident {}

impl crate::core::Plugin for Ident {
    fn positive() -> Vec<&'static str> {
        vec![r"[\p{XID_Start}_]\p{XID_Continue}*"]
    }

    fn negative() -> Vec<&'static str> {
        vec![r"^(?![\p{XID_Start}_]\p{XID_Continue}*$).*$"]
    }

    fn run(slice: &str) -> String {
        slice.to_string()
    }
}
