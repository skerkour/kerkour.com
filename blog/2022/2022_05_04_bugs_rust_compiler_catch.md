+++
date = 2022-05-04T02:00:00Z
title = "Bugs that the Rust compiler catches for you"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/bugs-rust-compiler-helps-prevent"

[extra]
lang = "en"

comment ="""
"""
+++

Over the decades, Humans [have](https://www.lunasec.io/docs/blog/log4j-zero-day-update-on-cve-2021-45046/) [proved](https://blog.ethereum.org/2016/06/17/critical-update-re-dao-vulnerability/) [to be](https://infosecwriteups.com/zoom-zero-day-4-million-webcams-maybe-an-rce-just-get-them-to-visit-your-website-ac75c83f4ef5) [pretty bad](https://www.henricodolfing.com/2019/06/project-failure-case-study-knight-capital.html) at [producing](https://www.searchenginejournal.com/vulnerability-found-in-wordpress-anti-malware-firewall/448101/) [bug-free](https://bugs.chromium.org/p/project-zero/issues/detail?id=1139) [software](https://www.openssl.org/news/secadv/20220315.txt). Trying to apply our approximative, fuzzy thoughts to perfectly logical computers seems doomed.

While the practice of code reviews is increasing, especially with the culture of Open Source becoming dominant, the situation is still far from perfect: it costs a lot of time and thus money.

What if, instead, we could have a companion, always available, never tired, and the icing on the cake, that doesn't cost the [salaray of a developer](https://insights.stackoverflow.com/survey/2021#work-salary) that would help us avoid bugs in our software before they reach production?

Let's see how a modern compiler and type system helps prevent many bugs and thus helps increase the security for everyone and reduces the costs of software production and maintenance.

## Resources leaks

It's so easy to forget to close a file or a connection:

```go
resp, err := http.Get("http://kerkour.com")
if err != nil {
    // ...
}
// defer resp.Body.Close() // DON'T forget this line
```

On the other hand, Rust enforces [RAII (Resource Acquisition Is Initialization)](https://doc.rust-lang.org/rust-by-example/scope/raii.html) which makes it close to impossible to leak resources: they automatically close when they are dropped.

```rust
  let wordlist_file = File::open("wordlist.txt")?;
  // do something...

  // we don't need to close wordlist_file
  // it will be closed when the variable goes out of scope
```


## Unreleased mutexes


Take a look at this Go code:

```go
type App struct {
  mutex sync.Mutex
  data  map[string]string
}

func (app *App) DoSomething(input string) {
  app.mutex.Lock()
  defer app.mutex.Unlock()
  // do something with data and input
}
```

So far, so good. but when we want to process many items, things can go very bad fast

```go
func (app *App) DoManyThings(input []string) {
  for _, item := range input {
      app.mutex.Lock()
      defer app.mutex.Unlock()
      // do something with data and item
  }
}
```

We just created a deadlock because the mutex lock is not released when expected but at the end of the function.

In the same way, RAII in Rust helps to prevent unreleased mutexes:

```rust
for item in input {
  let _guard = mutex.lock().expect("locking mutex");
  // do something
  // mutex is released here as _guard is dropped
}
```


## Missing switch cases

Let's imagine we are tracking the status of a product on an online shop:
```go
const (
  StatusUnknown   Status = 0
  StatusDraft     Status = 1
  StatusPublished Status = 2
)

switch status {
    case StatusUnknown:
        // ...
    case StatusDraft:
        // ...
    case StatusPublished:
        // ...
}
```

But then, if we add the `StatusArchived Status = 3` variant and forget to update this `switch` statement, the compiler still happily accepts the program and lets us introduce a bug.



While in Rust, a non-exhaustive `match` produces a compile-time error:

```rust
#[derive(Debug, Clone, Copy)]
enum Platform {
    Linux,
    MacOS,
    Windows,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::Macos => write!(f, "macOS"),
            // Compile time error! We forgot Windows and Unknown
        }
    }
}
```


## Invalid pointer dereference

As far as I know, it's not possible to create a reference to an invalid address in safe Rust.



```go
type User struct {
    // ...
    Foo *Bar // is it intended to be used a a pointer, or as an optional field?
}
```

And even better, because Rust has the [`Option`](https://doc.rust-lang.org/std/option/) enum, you don't have to use `null` pointer to represent the absence of something.


```rust
struct User {
    // ...
    foor: Option<Bar>, // it's clear that this field is optional
}
```

## Uninitialized variables

Let's say that we are processing users accounts:

```go
type User struct {
  ID          uuid.UUID
  CreatedAt   time.Time
  UpdatedAt time.Time
  Email       string
}

func (app *App) CreateUser(email string) {
    // ...
    now := time.Now().UTC()

    user := User {
      ID: uuid.New(),
      CreatedAt: now,
      UpdatedAt: now,
      Email: email,
    }
    err = app.repository.CreateUser(app.db, user)
    // ...
}
```

Good, but now, we need to add the field `AllowedStorage int64` to the `User` structure.

If we forget to update the `CreateUser` function, the compiler will still happily accept the code without any changes and use the default value of an `int64`: `0`, which may not be what we want.

While the following Rust code
```rust
struct User {
  id: uuid::Uuid,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  email: String,
  allowed_storage: i64,
}

fn create_user(email: String) {
    let user = User {
      id: uuid::new(),
      created_at: now,
      updated_at: now,
      email: email,
      // we forgot to update the function to initialize allowed_storage
    };
}
```

produces a compile-time error, preventing us from shooting ourselves in the foot.


## Unhandled exceptions and errors

It may sound stupid, but you can't have unhandled exceptions if you don't have exceptions...

[`panic!()`](https://doc.rust-lang.org/std/macro.panic.html) exists in Rust, but that's not how recoverable errors are handled.

Thus, by imposing the programmers to handle each and every errors (or the compiler refuses to compile the program), all while providing ergonomic tools to handle errors (the [`Result` enum](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) and the [`?`](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html) operator), the Rust compiler helps to prevent most (if not all) errors related to error handling.


## Data races

Thanks to the [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html) and [Send](https://doc.rust-lang.org/std/marker/trait.Send.html) traits, Rust's compiler can statically assert that no [data race](https://doc.rust-lang.org/nomicon/races.html) is going to happen.

How does it work? You can learn more in this [good write-up](https://medium.com/codex/eda-needs-to-be-using-rust-pt-2-59d2263ebb03) by Jason McCampbell.

<!-- ## Closures -->


## Hidden Streams

In Go, data streams are hidden behind the [`io.Writer`](https://pkg.go.dev/io#Writer) interface. On one hand, it allows to simplify their usage. On the other hand, it can reserve some surprise when used with types we don't expect to be a stream, a `bytes.Buffer` for example.

And that's exactly what happened to me a [month ago](https://kerkour.com/2022-03-23-newsletter-incident-postmortem): a `bytes.Buffer` was reused in a loop to render templates which led the templates to be appended to the buffer instead of the buffer to be cleaned and reused.

It would have never happened in Rust as [Streams](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) are a very specific type and would never have been used in this situation.


<!-- ## Memory leaks -->

<!-- (parler des edge cases where you leak memory) -->



## Some Closing Thoughts

Are smart compilers the end of bugs and code reviews?

Of course not! But a strong type system, and the associated compiler are a weapon of choice for anyone who wants to drastically reduce the number of bugs in their software and make their users / customers happy.
