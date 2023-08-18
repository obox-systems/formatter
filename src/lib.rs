use regex::Regex;

#[derive(Default)]
struct Builder {
    groups: Vec<Group>,
    regex: Vec<Vec<&'static str>>,
}

impl Builder {
    fn group(
        mut self,
        handler: impl Fn(&str) -> String + 'static,
        builder: impl FnOnce(&mut Vec<&'static str>),
    ) -> Self {
        self.regex.push(Vec::new());

        let stack = self.regex.last_mut().unwrap();
        builder(stack);

        self.groups.push(Group {
            handler: Box::new(handler),
        });

        self
    }

    fn finish(self) -> Formatter {
        let groups: Vec<String> = self
            .regex
            .iter()
            .map(|sublist| format!("({})", sublist.join("|")))
            .collect();

        let regex = groups.join("|");
        let regex = Regex::new(&regex).unwrap();

        Formatter {
            groups: self.groups,
            regex,
        }
    }
}

struct Group {
    handler: Box<dyn Fn(&str) -> String>,
}

struct Formatter {
    groups: Vec<Group>,
    regex: Regex,
}

impl Formatter {
    fn format(&self, input: &str) -> String {
        let formatted = self.regex.replace_all(input, |caps: &regex::Captures| {
            for (group, group_index) in self.groups.iter().zip(1usize..) {
                if let Some(n) = caps.get(group_index) {
                    return (group.handler)(n.as_str());
                }
            }

            caps[0].to_string()
        });

        formatted.to_string()
    }
}

pub fn format_code(input: &str) -> String {
    let formatted = Builder::default()
        // Adding spaces after "(" and before ")"
        // Adding spaces after "[" and before "]"
        // Adding spaces between operators
        .group(spaces, |group| {
            group.push(r"\(|\)");
            group.push(r"\[|\]");
            group.push(r"[-+*/%^&|<>=]");
        })
        // Adding a newline before {
        .group(newline, |group| group.push(r"\{"))
        .finish();

    formatted.format(input)
}

fn spaces(slice: &str) -> String {
    format!(" {slice} ")
}

fn newline(slice: &str) -> String {
    format!("\n{slice}\n{{")
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
