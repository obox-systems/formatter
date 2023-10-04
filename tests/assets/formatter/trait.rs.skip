#[enum_dispatch(DatabaseImpl)]
pub(crate) trait Database {
    fn add_question(&self, question: toml::Question) -> Result<()>;
    fn find_questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<toml::Question>>;
    fn migrations(&self) -> Result<()>;
}
