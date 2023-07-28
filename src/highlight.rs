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
    let mut output = String::with_capacity(input.len());

    let profile = std::fs::read_to_string("rust.toml").unwrap();
    let profile: ir::Profile = toml::from_str(&profile).unwrap();

    let lexer = ir::Lexer::new(profile.tokens);
    let mut stream = lexer.tokenize(input).stream();

    while let Some(token) = stream.next() {
        let color = lexer.color(token.kind);

        let slice = format!("{:?}", stream.slice());
        let slice = urlencoding::encode(&slice);

        output.push_str(&format_impl.highlight(color, &slice));
    }

    output
}
