use regex::Regex;

type CallbackList = Vec<fn(&str) -> String>;

pub(crate) trait Plugin {
    const PARTS: &'static [&'static str];

    fn run(slice: &str) -> String;
}

#[derive(Default)]
pub(crate) struct FormatterBuilder {
    pub(crate) callback_list: CallbackList,
    pub(crate) regex: Vec<&'static [&'static str]>,
}

impl FormatterBuilder {
    pub(crate) fn plugin<P: Plugin + 'static>(mut self) -> Self {
        self.regex.push(P::PARTS);
        self.callback_list.push(P::run);
        self
    }

    pub(crate) fn finish(self) -> Formatter {
        let groups: Vec<String> = self
            .regex
            .iter()
            .map(|sublist| format!("({})", sublist.join("|")))
            .collect();

        let regex = groups.join("|");
        let regex = Regex::new(&regex).unwrap();

        Formatter {
            callback_list: self.callback_list,
            regex,
        }
    }
}

pub(crate) struct Formatter {
    pub(crate) callback_list: CallbackList,
    pub(crate) regex: Regex,
}

impl Formatter {
    pub(crate) fn format(&self, input: &str) -> String {
        let formatted = self.regex.replace_all(input, |caps: &regex::Captures| {
            for (group, group_index) in self.callback_list.iter().zip(1usize..) {
                if let Some(n) = caps.get(group_index) {
                    return (group)(n.as_str());
                }
            }

            caps[0].to_string()
        });

        formatted.to_string()
    }
}

#[macro_export]
macro_rules! regex {
    ($($pattern:expr),*) => {
        const PARTS: &'static [&'static str] = &[$($pattern),*];
    };
}
