use fancy_regex::Regex;

type CallbackList = Vec<fn(&str) -> String>;

pub(crate) trait Plugin {
    /// Returns a vector of positive patterns that this plugin should match.
    fn positive() -> Vec<&'static str>;
    /// Returns a vector of negative patterns that this plugin should not match.
    fn negative() -> Vec<&'static str> {
        vec![]
    }
    /// Executes the plugin's logic on the given input slice and returns the result as a string.
    fn run(slice: &str) -> String;
}

#[derive(Default)]
pub(crate) struct FormatterBuilder {
    pub(crate) callback_list: CallbackList,
    pub(crate) regex: Vec<Vec<&'static str>>,
}

impl FormatterBuilder {
    pub(crate) fn plugin<P: Plugin + 'static>(mut self) -> Self {
        self.regex.push(P::positive());
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
        let regex = match Regex::new(&regex) {
            Ok(regex) => regex,
            Err(err) => {
                panic!("{err:?}: {regex}")
            }
        };

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
        let formatted = self
            .regex
            .replace_all(input, |caps: &fancy_regex::Captures| {
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
