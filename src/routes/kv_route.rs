use warp::{
    body::{content_length_limit, json},
    filters::BoxedFilter,
    path, Filter,
};

use crate::{
    json_body,
    models::pair::{Hdel, Hexist, Hget, Hmdel, Hmexist, Hmget, Hmset, Hset},
};
// use gamble::Hget;

use super::user_api_v1_path_prefix;

pub fn hget() -> BoxedFilter<(Hget,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hget"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hset() -> BoxedFilter<(Hset,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hset"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hgetall() -> BoxedFilter<(String,)> {
    warp::get()
        .and(user_api_v1_path_prefix())
        .and(path("hgetall"))
        .and(path::param::<String>())
        .and(path::end())
        // .and(json_body!())
        .boxed()
}

// pub fn hdel() -> BoxedFilter<(String,)> {
//     warp::delete()
//     .and(user_api_v1_path_prefix())
//     .and(path("hdel"))
//     .and(path::param::<String>())
//     .and(path::end())
//     // .and(json_body!())
//     .boxed()
// }

pub fn hmget() -> BoxedFilter<(Hmget,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hmget"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hmset() -> BoxedFilter<(Hmset,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hmset"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hdel() -> BoxedFilter<(Hdel,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hdel"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hmdel() -> BoxedFilter<(Hmdel,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hmdel"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hexist() -> BoxedFilter<(Hexist,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hexist"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

pub fn hmexist() -> BoxedFilter<(Hmexist,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("hmexist"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}
