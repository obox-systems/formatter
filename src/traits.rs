use wca::Context;

#[derive(Clone)]
pub(crate) struct Config {}

pub(crate) trait HasConfig {
    fn config(&self) -> &Config;
}

impl HasConfig for Context {
    fn config(&self) -> &Config {
        match self.get_ref() {
            Some(config) => config,
            None => {
                self.insert(load_config());
                self.get_ref().unwrap()
            }
        }
    }
}

fn load_config() -> Config {
    todo!()
}
