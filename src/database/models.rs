use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::database::init::Id;
use diesel::{deserialize, serialize, AsExpression, FromSqlRow, Identifiable, Queryable, QueryableByName, Selectable};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::SmallInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::error::AppError;

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Identifiable,
    Eq,
    PartialEq,
    Hash,
    Debug,
)]
#[diesel(table_name = crate::database::schema::blocklist)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blocklist {
    pub id: Id,
    pub ip: ipnetwork::IpNetwork,
    pub version: IpVersion,
    pub description: Option<String>
}


#[repr(i16)]
#[derive(
    AsExpression,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Copy,
    FromSqlRow,
    Hash,
)]
#[diesel(sql_type = SmallInt)]
pub enum IpVersion {
    Ipv4,
    Ipv6
}


impl Display for IpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IpVersion::Ipv4 => write!(f, "ipv4"),
            IpVersion::Ipv6 => write!(f, "ipv6"),
        }
    }
}

impl FromStr for IpVersion {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ipv4" => Ok(IpVersion::Ipv4),
            "ipv6" => Ok(IpVersion::Ipv6),
            _ => Err(AppError::ParseError(format!("Unknown IP version {}", s)))
        }
    }
}

impl Serialize for IpVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for IpVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        IpVersion::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<DB> FromSql<SmallInt, DB> for IpVersion
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match i16::from_sql(bytes)? {
            0 => Ok(IpVersion::Ipv4),
            1 => Ok(IpVersion::Ipv6),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

impl<DB> ToSql<SmallInt, DB> for IpVersion
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        match *self {
            IpVersion::Ipv4 => 0_i16.to_sql(out),
            IpVersion::Ipv6 => 1_i16.to_sql(out),
        }
    }
}