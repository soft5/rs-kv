use warp::{self, fs, path, Filter};

use crate::{
    session::user_session_filter,
    handlers::{
        user_handler,
        kv_handler,
        private::{
            profile_handler,
            password_handler,
        },
    },
    // Below are macros from api/
    routes::{
        user_route,
        kv_route,
        private::{
            profile_route,
            password_route,
        },
    },
    // From api/
    register_user,
    do_login,
    update_cash,
    get_profile,
    update_password,
    delete_user,
    logout,
    list_users,
    hget,
    hset,
    hgetall,
    hmget,
    hmset,
    hdel,
    hmdel,
    hexist,
    hmexist,
};

// Refer to them to make CORS work.
// https://github.com/seanmonstar/warp/blob/master/tests/cors.rs
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket

pub fn end() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // https://docs.rs/warp/0.2.2/warp/filters/fs/fn.dir.html
    // Use $RUST_LOG=warp::filters::fs=info cargo run --release
    // if you see the problem with this.
    // (https://github.com/steadylearner/Rust-Full-Stack/blob/master/React_Rust/server/warp/src/main.rs)
    let public_files = path("public")
        .and(fs::dir("./public/"))
        .with(warp::log("warp::filters::fs"));

    // https://docs.rs/warp/0.1.13/warp/filters/cors/fn.cors.html
    // https://github.com/seanmonstar/warp/blob/master/tests/cors.rs
    // Follow the example from others.
    // https://github.com/seanmonstar/warp/issues/361

    // https://docs.rs/warp/0.1.12/warp/trait.Filter.html#method.recover
    // Separate it with by GET / POST / PATCH / DELETE etc.
    // Then, separte it with auto required and others if necessary.
    register_user!()
        // .or(hello!())
        // .or(hi!())
        // It doens't work well with Parcel.
        // .or(register_user!().with(warp::cors().allow_any_origin().allow_method(Method::POST)))
        // .or(register_user!())
        .or(do_login!())
        // .or(authorized!())
        .or(get_profile!())
        .or(update_password!())
        .or(update_cash!())
        .or(delete_user!())
        .or(logout!())
        .or(list_users!())
        .or(hget!())
        .or(hset!())
        .or(hgetall!())
        .or(hmget!())
        .or(hmset!())
        .or(hdel!())
        .or(hmdel!())
        .or(hexist!())
        .or(hmexist!())
        .or(public_files)
}

pub fn routes_info() {
    let target: String = "0.0.0.0:8000".parse().unwrap();

    if !log_enabled!(log::Level::Info) {
        use console::Style;
        let blue = Style::new().blue();
        println!("\nRust Warp Server ready at {}\n", blue.apply_to(&target));
    }

    // info!("$curl 0.0.0.0:8000/hello/www.steadylearner.com to test the minimal end point.");
    // info!("$curl 0.0.0.0:8000/hi/www.steadylearner.com to test the Tera template views/");
    // info!("$curl 0.0.0.0:8000/public/rust_teloxide_example.png to test the ./public/ files.");

    // Test Ok(Success) parts with (CURL, frontend, tests/).
    info!("/ is to see the index(home) page.");
    info!("/api/user/v1 is to test user CRUD relevant routes.");
    // 1. Register (true, true, false)
    // 2. List (true, false, false)
    // 3. Delete user (true, false, false)
    info!("/api/user/v1/login is to test login relevant routes.");
    // POST - Login (true, true, false)
    // The path below are only allowed for who already did login.

    info!("/api/user/v1/authorized is to test the auth required page after a user do login.");

    info!("/api/user/v1/password is to update the password.");
    // PATCH - Update Password (curl, false, false)
    info!("/api/user/v1/logout is to test the logout.");
    // Get - Logout (true, true, false)
}
