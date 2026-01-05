use crate::database::error::DbError;
use crate::database::models::{Blocklist, BlocklistCreate};
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
use itertools::Itertools;

pub async fn get_ip(
    DbConnection(mut conn): DbConnection,
    params: Query<BlocklistIpVersion>,
) -> Result<impl IntoResponse, AppError> {
    let ips = blocklist::table
        .select(Blocklist::as_select())
        .filter(blocklist::version.eq(params.ip_version))
        .load::<Blocklist>(&mut conn)
        .await
        .map_err(DbError::from)?
        .into_iter()
        .map(|ip| ip.ip)
        .join("\n");
    Ok(ips)
}

pub async fn add_ip(
    DbConnection(mut conn): DbConnection,
    params: Json<BlocklistIp>,
) -> Result<(), AppError> {
    let ip = BlocklistCreate::from(params.0);
    diesel::insert_into(blocklist::table)
        .values(ip)
        .execute(&mut conn)
        .await
        .map_err(DbError::from)?;

    Ok(())
}
