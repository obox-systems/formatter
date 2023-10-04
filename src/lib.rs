mod core;
mod plugins;

pub fn format_code(input: &str) -> String {
    let formatter = core::FormatterBuilder::default()
        // Adding spaces after "(" and before ")"
        .plugin::<plugins::Parentheses>()
        // Adding spaces after "[" and before "]"
        .plugin::<plugins::Bracket>()
        // Adding a newline before {
        .plugin::<plugins::Braces>()
        // Adding spaces between operators
        .plugin::<plugins::Operators>()
        .finish();

    formatter.format(input)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::format_code;

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

    fn traverse(root: &str, expected_extension: &str, mut f: impl FnMut(PathBuf, PathBuf)) {
        for entry in std::fs::read_dir(root).unwrap() {
            let input_path = entry.unwrap().path();
            let expected_path = with_extension(&input_path, expected_extension);

            {
                let extension = input_path.extension().unwrap_or_default();
                if extension == "skip" || extension == expected_extension {
                    continue;
                }
            }

            f(input_path, expected_path)
        }
    }

    #[test]
    fn formatter() {
        let mut errors = Vec::new();

        traverse(
            "tests/assets/formatter",
            "expected",
            |input_path, expected| {
                let input = std::fs::read_to_string(&input_path).unwrap();
                let input = crate::format_code(&input);

                let expected = read_or_create(expected, &input);

                if input != expected {
                    if false {
                        std::fs::rename(&input_path, &with_extension(&input_path, "skip")).unwrap();
                    }

                    errors.push(input_path);
                }
            },
        );

        for error in &errors {
            println!("failed {}", error.display());
        }

        if !errors.is_empty() {
            std::panic::resume_unwind(Box::new(42));
        }
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

    #[test]
    fn test_case_1() {
        assert_eq!("f1( ){}", format_code("f1( ){}"));
    }

    #[test]
    fn test_case_2() {
        assert_eq!("f1( ){}", format_code("f1( ){}"));
    }

    #[test]
    fn test_case_3() {
        assert_eq!("f1(){}", format_code("f1(){}"));
    }

    #[test]
    fn test_case_4() {
        assert_eq!("let x : int = 3", format_code("let x:int=3"));
    }

    #[test]
    fn test_case_5() {
        assert_eq!(
            "f1( x ) , f2( y ) , f3()",
            format_code("f1( x ),f2(y) , f3()")
        );
    }

    #[test]
    fn test_case_6() {
        assert_eq!("[ 1, b, 3 ]", format_code("[1,b,3]"));
    }

    #[test]
    fn test_case_7() {
        assert_eq!("{ a : 1 , b : 2 }", format_code("{a:1,b:2}"));
    }
}
