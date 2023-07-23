use crate::ir;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(HighlighterImpl)]
pub(crate) trait Highlighter {
    fn highlight(&self, color: &str, slice: &str) -> String;
}

#[enum_dispatch]
pub(crate) enum HighlighterImpl {
    Markdown(Markdown),
}

pub(crate) struct Markdown;

impl Highlighter for Markdown {
    fn highlight(&self, color: &str, slice: &str) -> String {
        format!("![](https://img.shields.io/static/v1?label=&message={slice}&color={color})")
    }
}

#[allow(dead_code)]
pub(crate) fn highlight(input: &str, format_impl: HighlighterImpl) -> String {
    let mut output = String::new();

    let profile = std::fs::read_to_string("rust.toml").unwrap();
    let profile: ir::Profile = toml::from_str(&profile).unwrap();

    let state = ir::World::new(profile);
    let mut reader = state.tokenize(input).stream();

    while let Some(token) = reader.next() {
        let color = state.color(token.kind);

        let slice = format!("{:?}", reader.slice());
        let slice = urlencoding::encode(&slice);

        output.push_str(&format_impl.highlight(color, &slice));
    }

    output
}
