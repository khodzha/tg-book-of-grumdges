CREATE TABLE user_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    chat_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    rating BIGINT NOT NULL DEFAULT 0
)