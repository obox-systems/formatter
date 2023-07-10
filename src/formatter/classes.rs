pub(crate) use unicode_ident::{is_xid_continue, is_xid_start};

/// Source: `https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L276C1-L303C2`
pub(crate) fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        | '\u{0020}' // space

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// Source: `https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L276C1-L303C2`
pub(crate) fn is_newline(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        // NEXT LINE from latin1
        | '\u{0085}'
    )
}
