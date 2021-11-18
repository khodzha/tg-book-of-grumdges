use super::DbPool;

use diesel::expression_methods::*;
use diesel::query_dsl::*;
use diesel::Queryable;
use tokio::task::JoinHandle;

use crate::schema::user_ratings::dsl::*;

#[derive(Queryable)]
pub struct UserRating {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub rating: i64,
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

    pub fn execute_inc(&self, pool: DbPool) -> JoinHandle<diesel::QueryResult<i64>> {
        let chat = self.chat_id;
        let user = self.user_id;
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().unwrap();

            let r = diesel::insert_into(user_ratings)
                .values((chat_id.eq(chat), user_id.eq(user), rating.eq(1)))
                .on_conflict((chat_id, user_id))
                .do_update()
                .set(rating.eq(rating + 1))
                .returning(rating)
                .get_result(&conn)?;

            Ok(r)
        })
    }

    pub fn execute_dec(&self, pool: DbPool) -> JoinHandle<diesel::QueryResult<i64>> {
        let chat = self.chat_id;
        let user = self.user_id;

        tokio::task::spawn_blocking(move || {
            let conn = pool.get().unwrap();

            let r = diesel::insert_into(user_ratings)
                .values((chat_id.eq(chat), user_id.eq(user), rating.eq(-1)))
                .on_conflict((chat_id, user_id))
                .do_update()
                .set(rating.eq(rating - 1))
                .returning(rating)
                .get_result(&conn)?;

            Ok(r)
        })
    }
}
