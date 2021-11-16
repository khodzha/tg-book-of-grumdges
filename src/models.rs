use super::SqlitePool;

use diesel::query_dsl::RunQueryDsl;
use diesel::sql_types::BigInt;
use diesel::Queryable;
use tokio::task::JoinHandle;

#[derive(Queryable)]
pub struct UserRating {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub rating: i64,
}

#[derive(QueryableByName)]
struct UpsertedRating {
    #[sql_type = "BigInt"]
    rating: i64,
}

pub struct UpdateQuery {
    chat_id: i64,
    user_id: i64,
}

impl UpdateQuery {
    pub fn new(chat: i64, user: i64) -> Self {
        Self {
            chat_id: chat,
            user_id: user,
        }
    }

    pub fn execute_inc(&self, pool: SqlitePool) -> JoinHandle<diesel::QueryResult<i64>> {
        let chat = self.chat_id;
        let user = self.user_id;
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().unwrap();

            let r: UpsertedRating = diesel::sql_query(
                r#"
                    INSERT INTO user_ratings (chat_id, user_id, rating)
                    VALUES (?, ?, ?) ON CONFLICT(chat_id, user_id)
                    DO UPDATE SET rating = rating + 1 RETURNING rating
                "#,
            )
            .bind::<BigInt, _>(chat)
            .bind::<BigInt, _>(user)
            .bind::<BigInt, _>(1)
            .get_result(&conn)?;

            Ok(r.rating)
        })
    }

    pub fn execute_dec(&self, pool: SqlitePool) -> JoinHandle<diesel::QueryResult<i64>> {
        let chat = self.chat_id;
        let user = self.user_id;

        tokio::task::spawn_blocking(move || {
            let conn = pool.get().unwrap();

            let r: UpsertedRating = diesel::sql_query(
                r#"
                    INSERT INTO user_ratings (chat_id, user_id, rating)
                    VALUES (?, ?, ?) ON CONFLICT(chat_id, user_id)
                    DO UPDATE SET rating = rating - 1 RETURNING rating
                "#,
            )
            .bind::<BigInt, _>(chat)
            .bind::<BigInt, _>(user)
            .bind::<BigInt, _>(-1)
            .get_result(&conn)?;

            Ok(r.rating)
        })
    }
}
