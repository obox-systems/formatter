#[enum_dispatch]
pub(crate) enum DatabaseImpl {
    Sqlite(Sqlite),
    Memory(Memory),
}
