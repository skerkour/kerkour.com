## [How to implement HTTP Long Polling in Rust](https://kerkour.com/blog/rust-http-long-polling)

In shell 1:
```shell
$ docker run --name rust_long_polling -d -e POSTGRES_USER=rust -e POSTGRES_PASSWORD=long_polling -p 5432:5432 postgres:latest
$ DATABASE_URL=postgres://rust:long_polling@localhost:5432/rust cargo run
```


In shell 2:
```shell
$ curl http://localhost:8080/messages
```


In shell 3:
```shell
$ curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"body":"Hello World"}' \
  http://localhost:8080/messages
```
