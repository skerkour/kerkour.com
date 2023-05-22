+++
date = 2022-06-01T02:00:00Z
title = "When Serverless really shines (and when to avoid it)"
# title = "Good (and bad) use cases for serverless"
type = "post"
tags = ["programming", "devops", "serverless"]
authors = ["Sylvain Kerkour"]
url = "/when-to-use-serverless"

[extra]
lang = "en"

comment ="""
"""
+++

<!--
Titles:
- Serverless: Where it shines and when to avoid
- Useful applciation of serverless functions
- When serverless shine
- Good uses cases for serverless
- When to use serverless
-->
For a few years, serverless has been pushed hard by the different cloud providers as a magical solution for all our problems.

Before we get too far, let's clarify things. Here we are talking about serverless functions: [AWS lambda](https://aws.amazon.com/lambda) or [Scaleway Serverless Functions](https://www.scaleway.com/en/serverless-functions), where you upload a simple `.zip` file with your code or compiled program and pay per GB/seconds (of RAM/CPU) of usage.

While some services and products such as [serverless.com](https://www.serverless.com) and [Laravel Vapor](https://vapor.laravel.com) try to fit classic web services in this new paradigm, I think this may actually be one of the worst use cases for serverless functions and are better served by traditional PaaS such as [Heroku](https://www.heroku.com), [Render](https://render.com) or [Fly.io](https://fly.io).

Let's see a few examples of when NOT to use serverless functions and some clever usage of this pattern I observed in the wild.


## Why serverless

Managing servers is painful: managing certificates, security updates, scaling with the demand, firewalls, zero downtime deployments... Many things can and will go wrong.

By embracing serverless, developers delegate these tasks to the cloud platforms that provide fully managed environments. We just need to send the code, compiled or not, and they take care of everything else.

But, this value proposition is already offered by [PaaS](https://en.wikipedia.org/wiki/Platform_as_a_service) vendors.

The biggest differentiator of serverless functions is "Infinitely" scalable compute.

The idea behind the serverless paradigm is to decouple computing from storage (Databases and Object Storage - S3), so organizations should be able to always use exactly what they really need, as opposed to renting over-powerful instances with a lot of unused compute and storage to handle occasional traffic spikes.

This is why serverless offers come with a **pay-as-you-go** business model: you pay only for what you use (CPU time, RAM and storage). But, as we will see, it can be a huuuge drawback. **Serverless costs scale with your usage, not your revenues.**


## When serverless functions are a bad fit


### Sustained and stateful workloads (such as web services or workers)

The first rule of web development is to never trust users' input. So if you let the world decide for your usage (how many HTTP requests), and thus your costs, you are trusting users' input to not cost you too much.


Grave erreur, my dear. When it's not malicious users who try to bankrupt you, it can be a simple programming mistake. Here are a few write-ups of people almost going bankrupt due to the serverless' pay-as-you-go model:
- [Serverless: A lesson learned. The hard way](https://sourcebox.be/serverless-a-lesson-learned-the-hard-way/)
- [How we spent 30k USD in Firebase in less than 72 hours](https://hackernoon.com/how-we-spent-30k-usd-in-firebase-in-less-than-72-hours-307490bd24d)
- [We Burnt $72K testing Firebase + Cloud Run and almost went Bankrupt](https://blog.tomilkieway.com/72k-1/)

Here is how to basically *Denial-of-Wallet* any serverless-backed web service:
```bash
$ sudo apt install apache2-utils
$ while true; do ab -n 1000000 -c 1000 https://<URL>; done
```


And this is before talking about egress (bandwidth) fees and technical limitations, such as [databases with a limited number of connections](https://wiki.postgresql.org/wiki/Number_Of_Database_Connections), poor local development experience, cold starts, the latency induced by multiple inter-services calls for each request, vendor lock-in and so on...


So, please, don't do that.

<!-- find many stories of serverless costing too much -->

<!-- ### Calling many HTTP endpoints for a sinple CRUD app si so sloooowww -->

<!-- ici, parler de la latence de call plusieurs micro-functions -->

<!-- ### Too many serverless functions can DDOS your database -->

<!-- Ici parler des trop nombreuses connections a la DB et du becoins d'utilsier PG pool ou des choses similaire... -->

<!--
### Poor developer experience

While you no longer manage servers, reducing your needs in devops, the productivity of your developers suffer:
- The tooling is often vendor specific
- lack of good debugging tools
- sometimes lack lcoal development tools


resulting


<!-- Ici parler du vendor lockin


That's why I think that serverless functions are a very bad fit for users' facing applications, too many things can go wrong. -->


## Clever usage of serverless in the wild

There are, however, a few use cases where the use of serverless can drastically reduce costs, improve performance and simplify operations.


### Big data search

Serverless functions can replace Elasticsearch cluster for everything that is big data, low throughput, and not-so-low latency.

The perfect example is log storage and search. You don't need to search in your logs X Millions times a day. Only a few hundred at best. So instead of configuring and maintaining a large full-text search cluster such as Elasticsearch, it is cheaper to store all your raw logs on object storage (S3), and then parse and grep through your logs on demand.

We can easily parallelize the search using serverless functions, which is compute-intensive. For example, we can use 1 serverless function to parse and search in 1 day worth of logs, thus if our search range is 10 days, we would launch 10 parallel functions and aggregate the results, for 100 days, 100 parallel functions...

Given that a function would take less than 1 second to parse 1 day worth of log, by parallelizing the search, the total time to answer the query remains under 1 second for any date range.

![Serverless logs](https://kerkour.com/2022/when-to-use-serverless/serverless_logs.jpg)


Such a pipeline is extremely cheap, as the ingestor can be a small server, and then you only pay for object storage because your serverless function usage will certainly remain under the free-tier. No need to pay for always-running full-text search clusters.


### Sanboxed workloads

[Workload isolation is hard](https://fly.io/blog/sandboxing-and-workload-isolation/).

By using serverless functions, sandboxing becomes the job of the cloud platform, not yours, and they have great incentives to provide strong workload isolation.

A few examples where strong isolation is needed:
- Malicious file scanning
- Untrusted (such as user-provided) code execution
- Media files processing (because parsing complex formats is often the source of a lot of vulnerabilities)


### Global stateless deployments

Finally, a good use case for serverless is the global deployment of your functions when you don't need a data store and can embed the data that your app needs directly in the source code.

By deploying your functions globally, you can serve requests from your users all around the world extremely fast.

A good example is image processing or GeoIP: returning some metadata such as a country for a given IP address. The database is a few hundred MB and can be shipped beside the source code.






<!--
### Workers

But, be careful that serverless functions may outscale your downstream services such as a database with [limited sonnections](https://wiki.postgresql.org/wiki/Number_Of_Database_Connections), so you may want to put some concurrency limits. -->


## Some Closing Thoughts

While serverless alleviates the need to maintain servers, it often adds a lot of complexity on the development side.

Don't believe the people who have a vested interest in making you think that it's a magical solution that is going to solve all your pains. It's false.

Serverless is a niche pattern for high-compute, low throughout applications. That's all. For sustained workloads such as web services or workers, prefer a good old server that won't lead you to bankruptcy.
