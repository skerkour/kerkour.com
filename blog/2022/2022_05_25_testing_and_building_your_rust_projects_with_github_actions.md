+++
date = 2022-05-25T02:00:00Z
title = "Testing and building your Rust project with GitHub Actions"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-github-actions-ci-cd"

[extra]
lang = "en"

comment ="""
"""
+++


![A CI pipeine](https://kerkour.com/black-hat-rust/assets/ch04_ci.png)

Tests are not meant to be manually run each time you write code. It would be a bad usage of your precious time. Indeed, Rust takes (by design) a loooong time to compile. Running tests on your own machine more than a few times a day would break your focus.

Instead, tests should be run from CI (Continuous Integration). CI systems are pipelines you configure that will run your tests each time you push code. Nowadays practically all code platforms ([GitHub](https://github.com/features/actions), [GitLab](https://docs.gitlab.com/ee/ci/), [sourcehut](https://man.sr.ht/builds.sr.ht/)...) provide built-in CI. You can find examples of CI workflows for Rust projects here: [https://github.com/skerkour/phaser/tree/main/.github/workflows](https://github.com/skerkour/phaser/tree/main/.github/workflows).

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


```yaml
name: CI

# This workflow run tests and build for each push

on:
  push:
    branches:
      - main
      - 'feature-**'

jobs:

  test_phaser:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Update local toolchain
        run: |
          rustup update
          rustup component add clippy
          rustup install nightly

      - name: Toolchain info
        run: |
          cargo --version --verbose
          rustc --version
          cargo clippy --version

      - name: Lint
        run: |
          cd phaser
          cargo fmt -- --check
          cargo clippy -- -D warnings

      - name: Test
        run: |
          cd phaser
          cargo check
          cargo test --all

      - name: Build
        run: |
          cd phaser
          cargo build --release
```
