+++
date = 2018-07-25T01:42:42+02:00
title = "Logging"
tags = ["go", "golang", "logging"]
type = "post"
authors = ["Sylvain Kerkour"]

[extra]
lang = "en"
+++

1. [Structured logging](#structured-logging)
2. [Interface](#interface)
3. [Performance](#performance)
4. [What to log](#what-to-log)
5. [Logs vs Metrics](#logs-vs-metrics)
6. [Implementation](#implementation)


---------------------------

As the term [log is ambigous](https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying), I want to clarify that we will talk about application logging.

<a href="https://blog.codinghorror.com/the-problem-with-logging/" target="_blank" rel="noopener noreferrer">Logging</a>
<a href="https://dave.cheney.net/2015/11/05/lets-talk-about-logging" target="_blank" rel="noopener noreferrer">is</a>
<a href="https://logmatic.io/blog/our-guide-to-a-golang-logs-world/" target="_blank" rel="noopener noreferrer">hard</a>.



I think logging is a critical piece of infrastructure. Chances are if you’re a software engineer, you’ll be staring at logs for half of your life, so it’s worth taking the time to discover the *"perfect"* logging solution.

Here you can find my contribution to make logging great again (building great products is hard enough, we should not struggle with things like logging).


## Structured logging

Strucured logging is the practice to see logs as a dictionnay of key/value pairs.

Im not sure when structured logging was introduced but one of the first iteration was the [logfmt](https://brandur.org/logfmt) format.

Nowadays everyone agree that structured logging should be done in **JSON**.

It ensure that logs are **machine friendly** (for searching) and can also be easily formatted to be human friendly and terminate the *grep pain*.

*Logs are for humans after being processed by machines (indexing, formatting...).*


#### Message

Logging is about events that happened thus the best way to write a log **message** is to use the past.

A good way to describe past events is to use the tupple (target, action), with **target** being a noun and **action** a verb in it preterit form eg: "server started".



## Interface

#### Levels

I love great design so I love clean interfaces. A clean interface should be minimal.

Rapported to logging that mean we should reduce the number of methods and logging levels.

Here are the acceptable levels:

- **Debug**
- **Info**
- **Warning**
- **Error**
- **Fatal** *log with the fatal level, then exit(1)*

Your team should spec a logging style before starting to code and be consistent. This will help you digging in your logs when a problem will happen (and it will). [example](https://stackoverflow.com/questions/153524/code-to-logging-ratio#153547)


#### Context and fields

As we said structured logging is about logging key value pairs.

**fields** are key/value pairs.

A clean interface to add fields is **With(fields...)**.

Thus a simple log line with additional fields should be (language agnostic):

```javascript
log.With({key_one: "value1", key_two: "value2"}).Info("something happened")
// {"key_one": "value1", "key_two": "value2", "message": "something happened", "level": "info", "timestamp": "2018-07-25T08:42:05.39Z"}
```

It should also be possible to create a contextual sublogger from a logger and contextual fields:

```javascript
sublogger := log.With({request_id: uuid})
// then
sublogger.With({key_one: "value1", key_two: "value2"}).Info("something else happened")
// {"request_id": "1234", "key_one": "value1", "key_two": "value2", "message": "something happened", "level": "info", "timestamp": "2018-07-25T08:42:05.39Z"}
```

#### Events

Events (a.k.a *entrie* or *records*) are each individual line of log.

Usually they all contain the common fields **message**, **level** and **timestamp**, but it's not mandatory.

The difference between an event in your logs and an event tracked by an analytics service should be: What meaning will be extracted from this event or aggregations of this type of events ?

- If it's a business insight, you should use you analytics tracker.
- Else (engineering insight) you should use you logging library.

Please remind they are not mutually exclusive (you can track an event with both systems if it makes sens).

#### Destination

A clean logger should have only one destination. It's where it ship its log events. In Go this is commonly done with the `io.Writer` interface.


#### Hooks

Optionnaly you can add a hooks to manipulate events (eg. to add feilds) before they are written to the destination.



## Performance
*"Logging is not free so I'm not sure if I should add this log line"*.

Logging is not free, but it can be [extremly](https://github.com/rs/zerolog) [fast](https://github.com/uber-go/zap). It's the logging library's responsibility.

Here is the foundation for a cheap logging library:

#### Few to zero allocations


#### Asynchronous

Asynchronous means that the logger does not write to it's *destination* for every log call, but maintains an internal buffer which is flushed (logs a written to the destination which is blocking) every x seconds or when the internal buffer is full.

The main concern is to provide a great interface to **flush** the logger's buffer before ending the program, thus `log.Error` and `log.Fatal`  should block and flush the buffer.


#### Sampling

When too many events in a short time window are the same, it can be ok to drop some to not saturate the system. It's sampling. The goal is only to have better performance when it's truly critical.


## What to log

The goal of logs is to provide **observability** to you applications and infrastructure.

You should log what will help you find errors in your program and give you insights in how your application behave.

**Logs should be used after fact, to understand how things happened.**

**Logs should not be used for monitoring, use metrics instead.**




## Logs vs Metrics

As we seen logs are structured events. On the other hand metrics are numerics time series (which you can add some labels / key value which is a kind of structure, [metrics](https://github.com/prometheus/prometheus) [stores](https://github.com/influxdata/influxdb) are optimized for numeric time series).

If the goal of metrics is to provide **observability** to you applications and infrastructure - like logs - they are a preferred mean for monitoring and alerting because of their numeric nature which ease aggregations and graphing.

Please remind that some metrics can be derived from logs (mean http request duration, http request rate, error rate...).

As it's out of topic, [here is more information](https://medium.com/@copyconstruct/logs-and-metrics-6d34d3026e38#5d89).




## Implementation

I created the framework [Astro](https://bloom.sh/open-source) which will have implementations for all the languages I use (contributions are welcome for other languages!) with respect to the above principles.

The masterplan of astro is to provide an universally (for all languages) consistent high performance logging library without compromise on the UX so with a clean API.
