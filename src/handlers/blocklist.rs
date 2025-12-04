use crate::database::models::{Blocklist, IpVersion};
use crate::database::schema::blocklist;
use crate::forms::blocklist::BlocklistIpVersion;
use crate::pool::DbConnection;
use axum::extract::Query;
use axum::response::IntoResponse;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use crate::database::error::DbError;
use crate::error::AppError;

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
        .map(|ip|ip.ip)
        .join("\n");

    Ok(ips)
}
