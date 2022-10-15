#[macro_export]
macro_rules! hget {
    () => {
        kv_route::hget()
        .and(user_session_filter())
        .and_then(kv_handler::hget)
    };
}

#[macro_export]
macro_rules! hset {
    () => {
        kv_route::hset()
        .and(user_session_filter())
        .and_then(kv_handler::hset)
    };
}

#[macro_export]
macro_rules! hgetall {
    () => {
        kv_route::hgetall()
        .and(user_session_filter())
        .and_then(kv_handler::hgetall)
    };
}

#[macro_export]
macro_rules! hmget {
    () => {
        kv_route::hmget()
        .and(user_session_filter())
        .and_then(kv_handler::hmget)
    };
}

#[macro_export]
macro_rules! hmset {
    () => {
        kv_route::hmset()
        .and(user_session_filter())
        .and_then(kv_handler::hmset)
    };
}

#[macro_export]
macro_rules! hdel {
    () => {
        kv_route::hdel()
        .and(user_session_filter())
        .and_then(kv_handler::hdel)
    };
}

#[macro_export]
macro_rules! hmdel {
    () => {
        kv_route::hmdel()
        .and(user_session_filter())
        .and_then(kv_handler::hmdel)
    };
}

#[macro_export]
macro_rules! hexist {
    () => {
        kv_route::hexist()
        .and(user_session_filter())
        .and_then(kv_handler::hexist)
    };
}

#[macro_export]
macro_rules! hmexist {
    () => {
        kv_route::hmexist()
        .and(user_session_filter())
        .and_then(kv_handler::hmexist)
    };
}