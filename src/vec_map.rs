use m_lexer::TokenKind;

pub(crate) struct VecMap<T> {
    values: Vec<T>,
}

impl<T> VecMap<T> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        VecMap {
            values: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn insert(&mut self, value: T) -> TokenKind {
        let kind = TokenKind(self.values.len() as u16);
        self.values.push(value);
        kind
    }
}

impl<T> std::ops::Index<TokenKind> for VecMap<T> {
    type Output = T;

    fn index(&self, index: TokenKind) -> &Self::Output {
        &self.values[index.0 as usize]
    }
}
