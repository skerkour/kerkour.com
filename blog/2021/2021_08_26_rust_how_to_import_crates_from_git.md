+++
date = 2021-08-26T14:00:00Z
title = "How to import a crate from git in Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-import-crate-from-git"

[extra]
lang = "en"

comment ="""
"""
+++

Importing a crate from git is as simple as adding the following line in your **Cargo.toml** file:

```toml
[dependencies]
ring = { git = "https://github.com/briansmith/ring", version = "0.12" }
# or
ring = { git = "https://github.com/briansmith/ring", branch = "main" }
```
