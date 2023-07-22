use crate::ir;
use std::fmt::Write;

trait Format {
    fn format(&self, color: String, slice: &str);
}

// TODO: impl me
#[allow(dead_code)]
enum FormatImpl {
    Markdown,
    Ansi,
}

// only for testing
#[allow(dead_code)]
pub(crate) fn highlight(input: &str) -> String {
    let mut output = String::new();

    let profile = std::fs::read_to_string("rust.toml").unwrap();
    let profile: ir::Profile = toml::from_str(&profile).unwrap();

    let state = ir::World::new(profile);
    let mut reader = state.tokenize(input).reader();

    while let Some(token) = reader.next() {
        let color = state.color(token.kind);

        let slice = format!("{:?}", reader.slice());
        let slice = urlencoding::encode(&slice);

        writeln!(
            output,
            "![](https://img.shields.io/static/v1?label=&message={slice}&color={color})"
        )
        .unwrap();
    }

    output
}
