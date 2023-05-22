+++
date = 2021-02-24T12:42:42+01:00
title = "Rust for web development: 2 years later"
type = "post"
tags = ["rust", "bloom", "open-source", "web-dev"]
authors = ["Sylvain Kerkour"]
url = "/rust-for-web-development-2-years-later"

[extra]
lang = "en"
+++

I started developing web services (JSON APIs) in Rust a little bit more than 2 years ago, so I thought it was time to shake the preconceived ideas and share what I've learned.

<!-- {{< bhr_banner >}} -->

## The prejudices

**Rust code is ugly**: Rust is explicit. Undeniably. But when I write code, my IDE helps me a lot, and I don't have to press that many keys. When I read code, this explicitness is just awesome! No hidden surprises, no weird things.

**Memory management is a distraction**: Actually, no. I don't use that many lexical lifetimes and use instead smart pointers for long-lived objects such as a database connection pool. So yes, I have to understand the differences between an `Rc` and an `Arc`, but my productivity is not impacted compared to Node.JS or Go.

**Compiler is a pain in the a****: At the beginning, yes. But after a few months, I was able to understand all the errors instantly and fix them in a blink. Today I really don't spend much time fighting with the compiler. Instead, it became my best friend, especially when refactoring large portions of code or upgrading dependencies :)

**Slow compile times**: I accord you this one. When in Node.JS or Go a medium-sized service's Docker image takes 3 to 10 mins to build and deploy, in Rust it will be about 30 minutes.

**The ecosystem is not there**: Yet. Yes, some pieces are missing, such as official Stripe and AWS SDKs, but the community is really active and built all these missing pieces.

## A few things I particularly appreciate

**Static linking is remarkably easy**: Creating [small Docker images](https://github.com/skerkour/bloom/blob/main/Dockerfile) is a delight.

**Rust will make you a better programmer**: Rust is complicated and won't leave you alone if you don't understand how it works in detail. It takes time and patience to master, but once you do, you'll have learned so many things that you'll never approach programming as before. I understood how Go's runtime works while learning how [tokio](https://github.com/tokio-rs/tokio) works.

**Once it compiles, it works** (usually): This is my favorite thing about Rust. When my program compiles, it works as I planned. Just remember not to block the event loop, and the compiler will take care of the rest. You no longer have to spend time writing tests for the quirks of the language.

**Rust is productive**: Because Rust is multi-paradigm, it really shines when writing complex business logic, thanks to its functional aspects.

**Crates I use**:
* ~~[actix-web](https://github.com/actix/actix-web)~~ replaced by [axum](https://github.com/tokio-rs/axum) for the HTTP layer.
* [sqlx](https://github.com/launchbadge/sqlx) for the database (PostgreSQL).
* ~~[rusoto](https://github.com/rusoto/rusoto)~~ replaced by [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust) to interface with AWS services for storage (S3), background jobs (SQS) and sending emails (SES).
* [tera](https://github.com/Keats/tera) for email templates.
* [thiserror](https://github.com/dtolnay/thiserror) for my error types.
* [tracing](https://github.com/tokio-rs/tracing) and  [sentry](https://github.com/getsentry/sentry-rust) for instrumentation, logging and crash monitoring.


## Conclusion

Rust is a real pleasure to use for web development, and I thoroughly recommend to give it a try.
<!-- It is so versatile that, without a doubt, it's the last programming language I learn. Ever. -->

It's a <ins>loooong</ins> journey to become effective, but totally worth it, and even if you don't use it every day, you'll for sure become a better programmer by learning it and, if lost, rediscover the joy of programming 🤗

In a word: Rust brings calm. No more bad surprises at 3 A.M. No more bugs because a dependency updated its API. No more annoying configuration for auto-scaling or whatever. And welcome response times so small that your users will fall in love with your product.


