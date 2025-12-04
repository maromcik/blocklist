use crate::database::error::DbError;
use crate::database::models::{Blocklist, BlocklistCreate, IpVersion};
use crate::database::schema::blocklist;
use crate::error::AppError;
use crate::forms::blocklist::{BlocklistIp, BlocklistIpVersion};
use crate::pool::DbConnection;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use ipnetwork::IpNetwork;
use itertools::Itertools;
use log::info;
use tokio::fs;
use tracing::error;

pub async fn read_ip_set_file(path: &str) -> Result<String, AppError> {

            let data = fs::read_to_string(path).await.unwrap();
    Ok(data)
}

pub fn parse_from_string<S: AsRef<str>>(
    data: Option<S>,
    split_string: Option<&str>,
) -> Option<Vec<String>> {
    match data {
        Some(s) if !s.as_ref().is_empty() => match split_string {
            None => Some(
                s.as_ref()
                    .split_whitespace()
                    .map(|s| s.trim().to_string())
                    .collect(),
            ),
            Some(split_str) => Some(
                s.as_ref()
                    .split(split_str)
                    .map(|s| s.trim().to_string())
                    .collect(),
            ),
        },
        _ => None,
    }
}

pub async fn get_ip(
    DbConnection(mut conn): DbConnection,
    params: Query<BlocklistIpVersion>,
) -> Result<impl IntoResponse, AppError> {
    let ips= parse_from_string(Some(read_ip_set_file("blist.txt").await.expect("NOT FOUND")), None).expect("PARSE ERROR");
    let items: Vec<BlocklistCreate> = ips
        .into_iter()
        .map(|ip_str| BlocklistCreate {
            ip: ip_str.parse::<IpNetwork>().unwrap(), // or ? for error
            version: IpVersion::Ipv4,
            description: None,
        })
        .collect();
    let inserted = diesel::insert_into(blocklist::table)
        .values(&items)
        .on_conflict(blocklist::ip)       // ← ignore duplicates
        .do_nothing()
        .execute(&mut conn)
        .await
        .map_err(DbError::from)?;
    Ok("success")

    }


pub async fn add_ip(
    DbConnection(mut conn): DbConnection,
    params: Json<BlocklistIp>
) -> Result<(), AppError> {
    let version = match params.ip {
        IpNetwork::V4(_) => IpVersion::Ipv4,
        IpNetwork::V6(_) => IpVersion::Ipv6
    };
    let ip = BlocklistCreate {
        ip: params.ip,
        version,
        description: None,
    };
    diesel::insert_into(
        blocklist::table
    )
        .values(ip).execute(&mut conn).await.map_err(DbError::from)?;

    Ok(())
}