pub struct Hello {
    hello: u32,
    value: String,
}

pub struct Hello<'a> {
    hello: u32<'a>,
    value: String<'a>,
}
