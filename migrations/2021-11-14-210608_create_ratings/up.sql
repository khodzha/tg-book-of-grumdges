CREATE TABLE user_ratings (
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    rating BIGINT NOT NULL DEFAULT 0
)
