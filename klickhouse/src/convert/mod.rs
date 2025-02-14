use crate::{types::Type, Value};
use anyhow::*;

mod std_deserialize;
mod std_serialize;

/// A type that can be converted to a raw Clickhouse SQL value.
pub trait ToSql {
    fn to_sql(self) -> Result<Value>;
}

impl ToSql for Value {
    fn to_sql(self) -> Result<Value> {
        Ok(self)
    }
}

pub fn unexpected_type(type_: &Type) -> anyhow::Error {
    anyhow!("unexpected type: {}", type_.to_string())
}

/// A type that can be converted from a raw Clickhouse SQL value.
pub trait FromSql: Sized {
    fn from_sql(type_: &Type, value: Value) -> Result<Self>;
}

impl FromSql for Value {
    fn from_sql(_type_: &Type, value: Value) -> Result<Self> {
        Ok(value)
    }
}

/// A row that can be deserialized and serialized from a raw Clickhouse SQL value.
/// Generally this is not implemented manually, but using `klickhouse_derive::Row`.
/// I.e. `#[derive(klickhouse::Row)]`.
pub trait Row: Sized {
    fn deserialize_row(map: Vec<(&str, &Type, Value)>) -> Result<Self>;

    fn serialize_row(self) -> Result<Vec<(&'static str, Value)>>;
}
