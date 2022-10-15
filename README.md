# Rust Warp backend server

This is a Rust Warp backend kv server. Change from [Rust-Warp-Example](https://github.com/steadylearner/Rust-Warp-Example.git).

## How to test it

You can use **$python3 dev.py** or **$cargo run --bin main** or **$RUST_LOG=debug cargo run --bin main** to test a web server.

If you want to start simple, start with **hello** and **hi** apis.

You can also test other CLI commands with cargo run --bin name. Refer to **Cargo.toml** for that.

## End points

I let CURL commands for each files in routes/ folder to help you test the end points. But, you can start with these first.

* Register a user

```console
$curl -X POST localhost:8000/api/user/v1 -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'
```

* List users

```console
$curl localhost:8000/api/user/v1
```

* Login

```console
curl -X POST localhost:8000/api/user/v1/login -c cookie.txt -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'
```

* Update cash

```console
$curl -X PATCH localhost:8000/api/user/v1/cash -b cookie.txt -L -H "Content-Type: application/json" -d '{ "amount": 100000 }'
```

* Delete a user

```console
$curl -X GET localhost:8000/api/user/v1/logout -b cookie.txt -L
```


- hget
```console
curl -X POST localhost:8000/api/user/v1/hget -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1",  "key": "hello"}'
```


- hset
```console
curl -X POST localhost:8000/api/user/v1/hset -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1",  "key": "hello", "value":  "world"}'
```

- hgetall
curl -X POST localhost:8000/api/user/v1/hgetall -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1"}'
curl -X GET localhost:8000/api/user/v1/hgetall/table1 -b cookie.txt -L 
curl -X GET localhost:8000/api/user/v1/hgetall -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1"}'
```

- hmset
```console
curl -X POST localhost:8000/api/user/v1/hmset -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1", "pairs":  [ {"key": "hello", "value":  "world"},  {"key": "hello2", "value":  "world2"}]}'
```

- hmget
```console
curl -X POST localhost:8000/api/user/v1/hmget -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1", "keys":  ["hello", "hello2"]}'
```

- hdel
```console
curl -X POST localhost:8000/api/user/v1/hdel -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1", "keys":  ["hello"]}'
```

- hmdel
```console
curl -X POST localhost:8000/api/user/v1/hmdel -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1", "keys":  ["hello"]}'
```

- hexist
```console
curl -X POST localhost:8000/api/user/v1/hexist -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1", "keys":  ["hello"]}'
```

- hmexist
```console
curl -X POST localhost:8000/api/user/v1/hmexist -b cookie.txt -L -H "Content-Type: application/json" -d '{"table": "table1", "keys":  ["hello"]}'
```
