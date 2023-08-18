use regex::Regex;

pub fn format_code(input: &str) -> String {
    let re = Regex::new(r"(\(|\)|\[|\]|[-+*/%^&|<>=])|(\{)").unwrap();

    let formatted = re.replace_all(input, |caps: &regex::Captures| {
        if let Some(idx) = (1..=2).find(|i| caps.get(*i).is_some()) {
            match idx {
                1 => format!(" {} ", &caps[0]),
                2 => format!("\n{}\n{{", &caps[0]),
                _ => caps[0].to_string(),
            }
        } else {
            caps[0].to_string()
        }
    });

    formatted.to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    // use crate::highlight::Markdown;

    #[test]
    fn it_works() {
        let input = r#"
fn main(){
let x=[1,2,3];
if(x>5){
println!("x is greater than 5");
}
}
"#;

        let output = super::format_code(input);
        println!("{}", output);
    }

    fn update_expect() -> bool {
        std::env::var("UPDATE_EXPECT").is_ok()
    }

    fn with_extension(path: &PathBuf, extension: &str) -> PathBuf {
        match path.extension() {
            Some(raw_extension) => {
                let mut raw_extension = raw_extension.to_os_string();
                raw_extension.push(".");
                raw_extension.push(extension);
                path.with_extension(raw_extension)
            }
            None => path.with_extension(extension),
        }
    }

    fn read_or_create(path: PathBuf, fallback: &str) -> String {
        let fallback = || {
            println!("\x1b[1m\x1b[92mupdating\x1b[0m: {}", path.display());
            std::fs::write(&path, fallback).unwrap();
            fallback.to_owned()
        };

        match std::fs::read_to_string(&path) {
            Ok(value) => {
                if update_expect() {
                    return fallback();
                }

                value
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => fallback(),
            Err(err) => panic!("{err:?}"),
        }
    }

    fn traverse(root: &str, expected_ext: &str, f: impl Fn(PathBuf, PathBuf)) {
        for entry in std::fs::read_dir(root).unwrap() {
            let input_path = entry.unwrap().path();
            let expected_path = with_extension(&input_path, expected_ext);

            if input_path.extension().unwrap_or_default() == expected_ext {
                continue;
            };

            f(input_path, expected_path)
        }
    }

    // #[test]
    // fn highlight() {
    //     traverse("tests/assets/lex", "md", |input, expected| {
    //         let input = crate::highlight::highlight(
    //             &std::fs::read_to_string(input).unwrap(),
    //             Markdown.into(),
    //         );

    //         let expected = read_or_create(expected, &input);

    //         assert_eq!(input, expected);
    //     });
    // }

    #[test]
    fn formatter() {
        traverse("tests/assets/formatter", "expected", |input, expected| {
            let input = std::fs::read_to_string(input).unwrap();
            let input = crate::format_code(&input);

            let expected = read_or_create(expected, &input);

            assert_eq!(input, expected);
        });
    }

    #[test]
    fn with_extension_test() {
        let path = PathBuf::from("file.txt");
        let expected = PathBuf::from("file.txt.expected");
        assert_eq!(with_extension(&path, "expected"), expected);

        let path = PathBuf::from("dir/file");
        let expected = PathBuf::from("dir/file.expected");
        assert_eq!(with_extension(&path, "expected"), expected);

        let path = PathBuf::from("file");
        let expected = PathBuf::from("file.expected");
        assert_eq!(with_extension(&path, "expected"), expected);
    }
}
