#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use teloxide::{
    prelude::*,
    types::{ForwardKind, ForwardOrigin, MediaKind, MessageKind},
};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();

    migrations::run();
    let pool = build_pool();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, move |message| {
        let pool = pool.clone();
        async move {
            let cmd = extract_command(&message.update);
            if let Some(cmd) = cmd {
                let ReplyCmd {
                    chat_id,
                    user,
                    update,
                } = &cmd;
                let r = match update {
                    RatingUpdate::Inc => {
                        models::UpdateQuery::new(*chat_id, user.id).execute_inc(pool)
                    }
                    RatingUpdate::Dec => {
                        models::UpdateQuery::new(*chat_id, user.id).execute_dec(pool)
                    }
                };
                match r.await.expect("Failed to join task") {
                    Ok(rating) => {
                        message
                            .reply_to(format!(
                                "{} rating = {}",
                                user.mention().unwrap_or_else(|| user.full_name()),
                                rating
                            ))
                            .await?;
                    }
                    Err(e) => {
                        log::error!("query failed for cmd = {:?}, err = {:?}", cmd, e)
                    }
                }
            }
            respond(())
        }
    })
    .await;
}

fn build_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::new(database_url);
    let pool = Pool::builder()
        .min_idle(Some(5))
        .max_size(10)
        .build(manager)
        .expect("Failed to build pool");

    pool
}

fn extract_command(message: &Message) -> Option<ReplyCmd> {
    match &message.kind {
        MessageKind::Common(msg) => match &msg.media_kind {
            MediaKind::Text(text) => match &msg.forward_kind {
                ForwardKind::Origin(ForwardOrigin {
                    reply_to_message: Some(replied_message),
                }) => {
                    let author = msg.from.as_ref()?;
                    let replied_author = match &replied_message.kind {
                        MessageKind::Common(reply_msg) => reply_msg.from.as_ref()?.to_owned(),
                        _ => return None,
                    };

                    // Dont let the author rate themselves
                    if author.id == replied_author.id {
                        return None;
                    }

                    let update = if text.text.starts_with("+") {
                        RatingUpdate::Inc
                    } else if text.text.starts_with("-") {
                        RatingUpdate::Dec
                    } else {
                        return None;
                    };

                    log::debug!(
                        "{:?}, chat = {:?}, user = {:?}",
                        update,
                        message.chat,
                        replied_author,
                    );

                    let reply_cmd = ReplyCmd {
                        user: replied_author,
                        chat_id: message.chat.id,
                        update,
                    };

                    return Some(reply_cmd);
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }

    None
}

#[derive(Debug, Clone)]
struct ReplyCmd {
    chat_id: i64,
    user: teloxide::types::User,
    update: RatingUpdate,
}

#[derive(Debug, Clone, Copy)]
enum RatingUpdate {
    Dec,
    Inc,
}

pub mod migrations;
pub mod models;
pub mod schema;
