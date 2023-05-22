+++
date = 2021-07-27T06:00:00Z
title = "My Rust development workflow (after 2+ years)"
type = "post"
tags = ["rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/rust-development-workflow"

old_description = """
Rust takes a loooot of time to compile, even with incremental compilation. It's not rare that a small change leads to 2 or 3 minutes of compilation to test the change, which frustrates a lot of new rustaceans. It's a deliberate choice made by the language designers to favor runtime speed over compilation speed (it's why we love Rust: raw speed), and there is no magic incantation or cheat code to reduce this compilation time.

It took me some time to find a good workflow, which does not break my flow state (when I'm in the zone), so I thought it was worth sharing if it can save you time, dear reader.
"""

[extra]
lang = "en"

comment ="""
Titres:
* how_to_code_in_rust_workflow
* My Rust development workflow

enjeux:
Rust met du temps a compiler et il n;y a pas d'incantaiton magique qui va changer ca
j'ai mis un peu de temps a trouver mon workflow, donc je pensis qu'il etait interessant de partager, si cela pour te faire gagner un peu de toi a toi, cher lecteur :)

Il vaut mieux ... ou toujours nager a contre courant?

comme vous vous en doutez, nager toujours a contre courant est fatiguant pour rien. A la place il vaut mieux (reference tao... chinois, jujitsu).

Show HN / Twitter: My rust development workflow (after 2+ years full-time)
"""
+++


Rust takes a loooot of time to compile, even with incremental compilation. It's not rare that a small change leads to 2 or 3 minutes of compilation to test the change, which frustrates a lot of new rustaceans. It's a deliberate choice made by the language designers to favor runtime speed over compilation speed (it's why we love Rust: raw speed), and there is no magic incantation or cheat code to reduce this compilation time.

It took me some time to find a good workflow, which does not break my [flow state](https://en.wikipedia.org/wiki/Flow_(psychology)) (when I'm in the *zone*), so I thought it was worth sharing if it can save you time, dear reader.


But first, I have a question: What makes you happier, swimming against the current or embracing the flow and getting things done?


If you come from programming languages such as TypeScript or Go, where the compilation is extremely fast, you should be used to fast  `code -> manually test -> fix -> repeat` cycles. Once a project becomes serious (large) enough, it's simply not possible in Rust. But it's absolutely not a reason to give up all the awesome things that Rust is bringing us, as software developers and engineers.

Here is the workflow I've developed over the past few years of professional Rust development after I understood that swimming against the current only brings eternal frustration.


(Actually, there are some "magic" workarounds such a [sccache](https://github.com/mozilla/sccache) to speed up CI, but in my opinion they are too complex, and I **highly value simplicity**, so thank you but no).


## Use `rust-analyzer`

First and foremost, use [`rust-analyzer`](https://rust-analyzer.github.io/).

When I started software development with Rust, `rust-analyzer` was not an option, and Rust programming was torture.

Today, I think that `rust-analyzer` is one of the most crucial factors for Rust adoption and user base growth. It does not only provide code completion but also inline warnings and errors and a lot of other features that come in handy during your programming sessions.

Not using `rust-analyzer` will make you lost a lot of time and I believe is pure and simple masochism.

Keep in mind that, sometimes, `rust-analyzer` will not give the most helpful error messages, so if you can't fix an error just by reading `rust-analyzer` messages, switch to `cargo check` which will give far better information.


## Trust your instinct

The next most important thing not to break your flow is to trust your instinct.

Because even `rust-analyzer` and `cargo check` are sometimes slow (up to a dozen seconds), waiting for them to complete after every line of code was driving me crazy.

Now, I write a lot of code before looking at errors and warnings. Trusting my instinct also means that I do not manually test my programs every few minutes. My experience is that fast `code -> manually test -> fix -> repeat` cycles are not possible in Rust. Instead, I trust my code to be *mostly* correct and do not interrupt my flow with unimportant things.

Once my full function or trait is implemented, I start fixing warnings and errors.


## Write tests

Your code passes `cargo check`? Great! But we are Humans: we are all fallible.

So the next step is to write tests. The Rust compiler provides a lot of guarantees and forces us to handle all edge cases, but is no help against logic errors.

Some prefer to write tests even before writing the actual code ([TDD](https://en.wikipedia.org/wiki/Test-driven_development)), I personally prefer after, anyway, you have to write tests to **detect logic errors**.


## Embrace continuous integration

Running tests require to compile them, so as you have guessed, it's slow.

So the last (but not least) thing to do to be productive in Rust, is to embrace [continuous integration](https://en.wikipedia.org/wiki/Continuous_integration).

<!-- ![Continuous integration](/2021/rust-development-workflow/ci.svg) -->

<div class="center">
  <img title="Continuous integration" src="/2021/rust-development-workflow/ci.svg" height="300"/>
</div>

The goal is to **never run the tests on your own computer**, but instead let your CI bots run the tests for you (they are machines after all, machines are made to work for us), and then, at the end of the day, check if some tests are failing and fix them.


For a large Go or Node.js project, a CI pipeline should take between 5 and 10 minutes. With Rust, on the other hand, it takes between **15 and 45 minutes** to run a CI pipeline, depending on your project and the power of your CI servers.

So the most sensible thing to do is to **fix your failing tests only once a day**.

It's called **bundling**: instead of splitting my day (and thus my attention and your focus) in smaller cycles, my `code -> manually test -> fix -> repeat` cycle now lasts a whole day.


In the same way that checking your emails every hour will kill your productivity, waiting for your tests to run every hour will kill your effectiveness.


As a final note, it's crucial for me to fix tests at the end of the day as it's a non-creative task. All my creative energy, available at the beginning of the day, is dedicated to implementing new things, not fixing tests.

For GitHub you have [GitHub Actions](https://docs.github.com/en/actions) and for GitLab you have [GitLab CI/CD](https://docs.gitlab.com/ee/ci/).

Here is an example of a CI pipeline for one of my Rust projects: [phaser/.github/workflows/ci.yml](https://github.com/skerkour/phaser/blob/main/.github/workflows/ci.yml)


## Project maintenance

Thanks to `cargo` and the community, project maintenance is straightforward in rust. You'll need to install [`cargo-outdated`](https://github.com/kbknapp/cargo-outdated) and [`cargo-audit`](https://github.com/rustsec/rustsec):

```shell
$ cargo install -f cargo-audit
$ cargo install -f cargo-outdated
```

And then, once a week:

```shell
$ cargo update
$ cargo outdated
# manually update the outdated dependencies
$ cargo audit
```


Aaaand, that's all. Good Rusting :)
