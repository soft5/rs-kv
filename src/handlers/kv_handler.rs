use rs_kv::SledDb;
use rs_kv::{db::sqlite::SQLITEPOOL, CommandRequest, MemTable, Service, ServiceInner};
// use gamble::Hget;
// use gamble::models::pair::Hget;
use rs_kv::pb::abi::Kvpair;
use warp::{reject::custom, reply, Rejection, Reply};

use crate::{
    handlers::{INTERNAL_SERVER_ERROR, UNAUTHORIZED},
    models::pair::{Hdel, Hexist, Hget, Hmdel, Hmexist, Hmget, Hmset, Hset},
    session::UserSession,
};

// lazy_static!{
//     pub static ref SERVICE_REF: Service = {
//         let service = ServiceInner::new(MemTable::new()).into();
//         service
//     };
// }

lazy_static! {
    pub static ref SERVICE_REF: Service<SledDb> = {
        let service = ServiceInner::new(SledDb::new("/tmp/kvserver"))
            .fn_before_send(|res| match res.message.as_ref() {
                "" => res.message = "altered. Original message is empty.".into(),
                s => res.message = format!("altered: {}", s),
            })
            .into();
        service
    };
}

pub async fn hget(
    table_key: Hget,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!(
        "hget ---------hget table: {}, key: {}",
        table_key.table, table_key.key
    );

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hget(table_key.table, table_key.key);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    // let response = match SQLITEPOOL.get() {
    //     Ok(conn) => match UserList::list_public(&conn) {
    //         Ok(public_user_list) => Ok(reply::json(&public_user_list)),
    //         Err(e) => {
    //             error!("{:#?}", e);
    //             Err(custom(INTERNAL_SERVER_ERROR))
    //         }
    //     },
    //     Err(e) => {
    //         error!("{:#?}", e);
    //         Err(custom(INTERNAL_SERVER_ERROR))
    //     }
    // };
    response
}

pub async fn hset(
    table_key_value: Hset,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!(
        "hset ----table: {}, key: {}, value: {}",
        table_key_value.table, table_key_value.key, table_key_value.value
    );

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                // let service = ServiceInner::new(MemTable::new()).into();
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hset(
                    table_key_value.table,
                    table_key_value.key,
                    table_key_value.value.into(),
                );
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hmget(
    table_keys: Hmget,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!("hmget ----table: {}", table_keys.table);

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                // let service = ServiceInner::new(MemTable::new()).into();
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hmget(table_keys.table, table_keys.keys);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hgetall(
    table: String,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!("hgetall ----table: {}", table);

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                // let service = ServiceInner::new(MemTable::new()).into();
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hgetall(table);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hmset(
    table_pairs: Hmset,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!(
        "hmset ----table: {}, {:?}",
        table_pairs.table, table_pairs.pairs
    );

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                // let service = ServiceInner::new(MemTable::new()).into();
                let service = SERVICE_REF.clone();
                let mut kv_pairs: Vec<Kvpair> = Vec::new();
                for pair in &table_pairs.pairs {
                    kv_pairs.push(Kvpair::new(pair.key.clone(), pair.value.clone().into()));
                }
                let cmd = CommandRequest::new_hmset(table_pairs.table, kv_pairs);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hdel(
    table_key: Hdel,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!(
        "hdel ---------hget table: {}, key: {}",
        table_key.table, table_key.key
    );

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hdel(table_key.table, table_key.key);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hmdel(
    table_keys: Hmdel,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!("hmdel ----table: {}", table_keys.table);

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                // let service = ServiceInner::new(MemTable::new()).into();
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hmdel(table_keys.table, table_keys.keys);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hexist(
    table_key: Hexist,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!(
        "hdel ---------hget table: {}, key: {}",
        table_key.table, table_key.key
    );

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hexist(table_key.table, table_key.key);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn hmexist(
    table_keys: Hmexist,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    println!("hmdel ----table: {}", table_keys.table);

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                // let service = ServiceInner::new(MemTable::new()).into();
                let service = SERVICE_REF.clone();
                let cmd = CommandRequest::new_hmexist(table_keys.table, table_keys.keys);
                let cmd_res = service.execute(cmd);
                println!("{:?}", cmd_res);
                let json_res = format!("{:?}", cmd_res);
                Ok(reply::json(&json_res))
            } else {
                debug!(
                    "Fail to buy a car without authorization. Should redirect a user to /login."
                );
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}
