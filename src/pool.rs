use crate::database::error::DbError;
use crate::error::AppError;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel_async::{AsyncPgConnection, pooled_connection::bb8};

pub struct DbConnection(pub bb8::PooledConnection<'static, AsyncPgConnection>);

type Pool = bb8::Pool<AsyncPgConnection>;

impl<S> FromRequestParts<S> for DbConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(DbError::from)?;

        Ok(Self(conn))
    }
}
