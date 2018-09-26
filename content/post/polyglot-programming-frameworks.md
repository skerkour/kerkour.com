+++
date = "2018-09-25T1:42:42+02:00"
draft = false
title = "Polyglot programming frameworks"
tags = ["programming", "logging", "go", "golang", "javascript", "js"]
+++

1. [Polyglot systems](#polyglot-systems)
2. [Astroflow](#astroflow)

---


## Polyglot systems

Today's advent of containers, orchestration technologies and services oriented architectures
leads to the fact that we use more and more programming languages for the same application.

Thus programmers have to code in more than one programming language.

The problem is that each programming language have it's own set of best practices, paradigms and popular
frameworks which slows software developers to easily switch from one language to another one.

These (increasingly) polyglot environments create the need for a better polyglot developer experience.

This is why I would like to introduce the concept of **polyglot framework**.

**Polyglot framework**: a framework which provides the same set (or very similar) of APIs, and make
the same trade-offs for different programming languages.


By creating polyglot frameworks we will enable programmers to create homogeneous polyglot systems,
with a shared set of quality standards, abilities and performances.


## Astroflow

[Logging is hard but unavoidable](https://kerkour.com/post/logging/) thus I think one of the opportunity
for polyglot frameworks to shine is logging.

This is why I started the [Astroflow](https://astrocorp.net/astroflow) framework, an universal logging framework.

Astroflow's goal is to provide a configurable and as fast as possible logging framework with the minimal
API possile.


You can find the Go repo here: [https://github.com/astrocorp42/astroflow-go](https://github.com/astrocorp42/astroflow-go)

and the JavaScrit one here: [https://github.com/astrocorp42/astroflow-js](https://github.com/astrocorp42/astroflow-js)
