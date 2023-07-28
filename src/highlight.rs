use crate::ir;
use enum_dispatch::enum_dispatch;

/// A trait representing a Highlighter that applies syntax highlighting to a code slice using a specific color.
///
/// This trait defines the contract for a Highlighter, allowing different implementations to provide various
/// syntax highlighting strategies based on the given color and code slice.
///
/// # Example
///
/// ```ignore
/// use my_highlighter::{Highlighter, MyCustomHighlighter};
///
/// struct MyCode {
///     code: String,
///     color: String,
/// }
///
/// impl Highlighter for MyCode {
///     fn highlight(&self, color: &str, slice: &str) -> String {
///         // Implement your custom highlighting logic here
///         // using the provided color and code slice.
///         // Return the highlighted code as a String.
///         // For example, you can use regular expressions or custom logic to apply the syntax highlighting.
///         // Below is a dummy implementation that simply adds the color to the slice.
///         format!("{}{}", color, slice)
///     }
/// }
///
/// // Usage example:
/// let my_code = MyCode {
///     code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
///     color: "\u{001b}[31m".to_string(), // Red color code for ANSI terminal (escape code)
/// };
///
/// let highlighted_code = my_code.highlight(&my_code.color, &my_code.code);
/// println!("{}", highlighted_code);
/// ```
#[enum_dispatch(HighlighterImpl)]
pub(crate) trait Highlighter {
    /// Highlights the provided code slice using the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - A string representing the color to be applied to the code slice.
    /// * `slice` - The code slice that needs to be highlighted.
    ///
    /// # Returns
    ///
    /// A new `String` containing the syntax-highlighted code slice.
    fn highlight(&self, color: &str, slice: &str) -> String;
}

/// An enum representing various implementations of a highlighter.
///
/// The `HighlighterImpl` enum is used to define different highlighter implementations
/// for different file types or formats. Currently, it only supports Markdown files.
#[enum_dispatch]
pub(crate) enum HighlighterImpl {
    /// A highlighter implementation for Markdown files.
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
