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

    /// Checks if the given character `ch` is present in the current state of `self`.
    /// If `ch` is present, shifts the internal state of `self` and returns `true`,
    /// otherwise returns `false`.
    pub(crate) fn shift_if(&mut self, edible: impl Testable) -> bool {
        let is_present = self.matches(edible);
        if is_present {
            self.shift();
        }
        is_present
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

    pub(crate) fn matches(&self, edible: impl Testable) -> bool {
        self.peek().is_some_and(|ch| edible.test(ch))
    }
}

pub(crate) trait Testable {
    fn test(self, ch: char) -> bool;
}

impl Testable for char {
    fn test(self, ch: char) -> bool {
        self == ch
    }
}

impl<F: Fn(char) -> bool> Testable for F {
    fn test(self, ch: char) -> bool {
        self(ch)
    }
}
