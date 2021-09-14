## [How to implement HTTP Long Polling in Rust](https://kerkour.com/blog/rust-http-long-polling)

In shell 1:
```shell
$ cargo run
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
