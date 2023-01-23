## [How to implement HTTP Long Polling in Go](https://kerkour.com/blog/go-http-long-polling)

In shell 1:
```shell
$ go run *.go
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
