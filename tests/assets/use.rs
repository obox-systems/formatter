mod memory;
mod sqlite;

use enum_dispatch::enum_dispatch;
use rusqlite::{Connection, Result};

pub(crate) use self::memory::Memory;
pub(crate) use self::sqlite::Sqlite;
use crate::toml;