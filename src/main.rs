#![deny(unreachable_pub)]

mod ir;

fn main() {
    use std::fmt::Write;

    let mut output = String::new();

    let profile = std::fs::read_to_string("rust.toml").unwrap();
    let profile: ir::Profile = toml::from_str(&profile).unwrap();

    let state = ir::World::new(profile);
    let mut reader = state
        .tokenize(
            r##"
    
     "hel
     lo" 
     
    "##,
        )
        .reader();

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

    std::fs::write("out.md", output).unwrap();
}
