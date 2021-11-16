table! {
    user_ratings (id) {
        id -> Nullable<Integer>,
        chat_id -> BigInt,
        user_id -> BigInt,
        rating -> BigInt,
    }
}
