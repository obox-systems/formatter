#![deny(unreachable_pub)]

mod highlight;
pub mod ir;
mod vec_map;

pub fn format(input: &str, profile: ir::Profile) -> String {
    let lexer = ir::Lexer::new(profile.tokens);
    let mut tokens = lexer.tokenize(input).stream();

    let mut output = String::new();
    while let Some(token) = tokens.next() {
        let mut runnable = true;

        if let Some(prev) = tokens.prev() {
            for rule in &profile.rules {
                if rule.before == "any"
                    || lexer.kind(&rule.before) == prev.kind
                        && lexer.kind(&rule.after) == token.kind
                {
                    if runnable {
                        output.push_str(&rule.action);
                    } else {
                        dbg!("skip rule");
                        dbg!(rule);
                    }

                    runnable = false;
                }
            }
        }

        output.push_str(tokens.slice());
    }

    output
}
