use async_trait::async_trait;
use crate::models::User;
use axum::extract::FromRequestParts;
use axum::response::{IntoResponse, Response};
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionSqlitePool;
use http::status::StatusCode;
use http::request::Parts;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::marker::{Send, Sync};
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

pub async fn connect_to_database() -> SqlitePool {
    let connect_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .unwrap()
}

pub struct Session(
    pub AuthSession<
        User,
        i64,
        SessionSqlitePool,
        SqlitePool,
    >,
);

impl Deref for Session {
    type Target = AuthSession<User, i64, SessionSqlitePool, SqlitePool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<S: Sync + Send> FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        AuthSession::<
            User,
            i64,
            SessionSqlitePool,
            SqlitePool,
        >::from_request_parts(parts, state)
        .await
        .map(Session)
        .map_err(|_| AuthSessionLayerNotFound)
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl Error for AuthSessionLayerNotFound {}
impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSessionLayer was not found",
        )
            .into_response()
    }
}
