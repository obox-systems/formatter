use colored::Colorize;

use crate::formatter::input::Input;

pub(crate) fn highlight(input: &str) -> String {
    let mut output = String::new();
    let input = Input::of(input);

    for token in input.iter() {
        let color = token.color();
        let slice = input.slice().on_color(color).to_string();

        output.push_str(&slice);
    }

    output
}
