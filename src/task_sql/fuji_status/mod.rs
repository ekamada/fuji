

use std::{fmt};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

#[derive(Clone, Copy)]
pub enum FujiStatus {
    Open,
    Closed
}

impl fmt::Display for FujiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output_str = match &self {
            Self::Open => "Open",
            Self::Closed => "Closed"
        };
        write!(f, "{output_str}")
    }
}


impl ToSql for FujiStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match &self {
            FujiStatus::Open => "Open",
            FujiStatus::Closed => "Closed"
        };
        Ok(value.into())
    }
}

impl FromSql for FujiStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let value_str = value.as_str().unwrap();
        match value_str {
            "Open"   => Ok(FujiStatus::Open),
            "Closed" => Ok(FujiStatus::Closed),
            _ => Err(FromSqlError::InvalidType)
        }
    } 
}

