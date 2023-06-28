use std::str::Chars;

pub(crate) struct Cursor<'me> {
    chars: Chars<'me>,
    len: usize,
}

impl<'me> Cursor<'me> {
    pub(crate) fn new(source: &'me str) -> Self {
        Self {
            chars: source.chars(),
            len: source.len(),
        }
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub(crate) fn shift(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(crate) fn shift_while(&mut self, f: impl Fn(char) -> bool + Copy) {
        while self.peek().is_some_and(f) {
            self.shift();
        }
    }

    pub(crate) fn reset_len(&mut self) -> u32 {
        let new_len = self.chars.as_str().len();
        let len = (self.len - new_len) as u32;
        self.len = new_len;
        len
    }

    pub(crate) fn matches(&self, ch: char) -> bool {
        self.peek() == Some(ch)
    }
}
