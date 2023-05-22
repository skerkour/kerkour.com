enjeux: Rust n'est pas un langage simple, mais il apporte beaucoup de simplicite a l'architecture global des services grace a sa performance et reliabilite


I love to present myself as someone who highly value simplicity, and still I've chosen Rust as my weapon of choice to build all my projects, which is considered by many as the ugliest and hardest programming language, just after haskell and brainfuck. WTF 🤪?


For example, lets imagine that we are asked to parse an environment variable to an `int`. In Rsut it would be as simple as:

```rust
let port: u16 = env::var("PORT")
    .ok()
    .unwrap_or("8080")
    .parse()?
// ...
```

Pretty straightforward isn't it?

While in Go (praised for its simplicity):
```go
var port uint64
var err error

portStr := os.Getenv("PORT")
if len(portStr) == 0 {
    portStr = "8080"
}

port, err = strconv.ParseUint(portStr, 10, 64)
if err != nil {
  return nil, err
}
// ...
```
