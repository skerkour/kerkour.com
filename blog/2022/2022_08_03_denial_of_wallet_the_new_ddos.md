+++
date = 2022-08-03T01:00:00Z
title = "Denial of Wallet Attacks: The new (D)DoS in a Serverless world"
type = "post"
tags = ["hacking", "programming", "serverless"]
authors = ["Sylvain Kerkour"]
url = "/denial-of-wallet-attacks-the-new-ddos"

[extra]
lang = "en"
+++



<!-- Wether it be in the physical (protests) or digital world, where the availability of a service or an area is attacked  Denial of Service (DoS ) attacks always have been a cost effective way to inflict a lot of damages to an adversary.



Traditionnal (D)DoS attacks rely on the fact that systems run on limited reousrces, and that these resources can be overloaded by a bigger system, or by distributing the load on many smaller systems. The financial damages inflicted are directly related to unavaibility of the system, which can be rather big, for a retailer at the end of the year for example.

But, with the advent of the cloud and its pay-per-use business model, this trend is taking a dramatical turn. -->


<!-- Since the commercial web exist, denial of service always existed to ransom or put a competitor out of business. -->

<!-- First it was easy.

Then with the emergence of the cloud and its instance more powerful than home bandwidth we saw the emergence of DDoS botnets and DDoS for rent. -->




<!-- But what is becoming worse? It's now far easier to rent a small instance -->


<!-- ## Which providers to avoid -->

<!--
Montrer le cout d'une attaque avec wrk, depuis mon instance stardust


montrer les couts en fonction de chaque provider

-->

<!-- ## Possible mitigations

<!-- presenter les differents WAF offerts

mais indiquer que certains WAF sont la pour premunir des DDOS et donc ne bloquent pas ce genre d'attaque
https://fasterthanli.me/articles/i-won-free-load-testing
 -->


Denial of Service (DoS) attacks always have been the easiest way to inflict maximum financial damages without requiring advanced skills or techniques. With the advent of cloud computing, website owners can now deploy more resources than the attackers and gracefully handle these primitive attacks.

It led to the development of more advanced attacks requiring a botnet: Distributed Denial of Service (DDoS). A network of machines infected by a malware (bot) that attack when their master orders.

But the cloud is not magical nor free and comes (most of the time) with a pay-as-you-go business model. The rationale is that you only need to pay for the resources you use and that costs should scale with your revenues.

And this is where things are taking a dramatic turn for website owners.


## The *pay-as-you-go* model

Most cloud providers promise something along the line of infinite scalability for your services. You don't need to worry. They'll scale your servers according to the load, and your bill will scale accordingly.


But they seem to have forgotten the wisdom of the previous generation of web (PHP) developers: **the first rule of web development is to never trust users' input** (XSS, SQL, HTML... injections).


By using a pay-as-you-go provider to power your systems, you are implicitly trusting that users are not going to abuse it.

Grave erreur, my dear.

**With the  *pay-as-you-go* model, your costs scale with your usage, not your revenues**. And increasing your usage is easy for anyone with access to your service.

Here is what it takes to inflict $$$ damages to a service using "serverless functions" under the hood, with [wrk](https://github.com/wg/wrk):

```bash
$ while true; do  wrk -t10 -c1000 -d10m <URL>; done
```

Let's dig further.

Let's say that the request takes 100ms to complete, uses 256 MB of RAM, and the response size is 1MB. Here is how much traffic I can generate with my $8/m, unlimited egress (bandwidth) server:

```bash
$ wrk -t12 -c1000 -d1m  https://xxx.s3.eu-west-1.amazonaws.com/1000000.bin
Running 1m test @ https://xxx.s3.eu-west-1.amazonaws.com/1000000.bin
  12 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.67s   304.76ms   2.00s    84.80%
    Req/Sec    18.23     11.23    90.00     69.49%
  10951 requests in 1.00m, 10.65GB read
Requests/sec:    182.21
Transfer/sec:    181.51MB
```

So for 1 month we get:
- 182 * 2,628,288 seconds = ~**478,348,416** requests
- 181 * 2,628,288 seconds = ~**475,720,128 MB** = ~**475,720 GB** = ~**476 TB** transferred

Here is how much it will cost you if you use the cloud (far from a scientific measurement because many factors come into play, but to give an idea):

| **Service** | **Cost** |
| ------------ | -------- |
| AWS API Gateway (HTTP API) | $989 (requests) |
| AWS Cloudfront | $478 (requests) + $23,644 (egress) |
| AWS Cloudfront | $478 (requests) + $23,644 (egress) |
| AWS EC2 | $20 (instance) + $28,262 (egress) |
| AWS S3 | $191 (requests) + $28,262 (egress) |
| Cloudflare Argo | $47,577 (egress) |
| Cloudflare Workers | $72 (requests) |
| Deno Deploy | $10 (plan) + $952 (requests) + $142,686 (egress) |
| DigitalOcean App Platform | $47,472 (egress) |
| DigitalOcean Functions | ~ $221 (requests) |
| Netlify (Pro plan) | $261,250 (egress) + $99 (plan) |
| Scaleway Serverless Functions | ~ $143 (requests) |
| Scaleway Serverless Containers | ~ $79 (requests) |
| Vercel (Pro plan) | $190,000 (egress) + ~ $92,872 (requests) + $20 (plan) |


And the cherry on top of the cake is that some of these costs can add up if you combine multiple of these services (e.g.: Cloudflare Argo + AWS API Gateway + AWS Lambda).

As we can see, there is no need to rent a huge botnet or buy advanced exploits to bring the companies using (some of) these services to their knees.

While this attack is very primitive, I want you to see the problem from another angle.

Here are a few examples of people who **involuntarily** almost went bankrupt because of the pay-as-you-go model:
- [Serverless: A lesson learned. The hard way](https://sourcebox.be/serverless-a-lesson-learned-the-hard-way/)
- [How we spent 30k USD in Firebase in less than 72 hours](https://hackernoon.com/how-we-spent-30k-usd-in-firebase-in-less-than-72-hours-307490bd24d)
- [We Burnt $72K testing Firebase + Cloud Run and almost went Bankrupt](https://blog.tomilkieway.com/72k-1/)


These only were small programming mistakes. Now I want you to imagine how much time it would take for one of your competitors to create a small script that realistically emulates real users to abuse your serverless systems.

Or, as it often happens to this website, a buggy crawler start to make hundreds of thousands of requests a day.


## What to do instead

Instead, you should run your services on providers that use a fixed-fee subscription that will not lead you to bankruptcy when a random disgruntled person on internet doesn't like your website.


- [Scaleway Instances](https://www.scaleway.com)
- [Cloudflare (without add-ons)](https://www.cloudflare.com)

And also the following ones where you still need to pay after some limits are reached but are way cheaper  than the  alternatives (in the sense that it will be way harder to rack up a huge bill, but the quality is still there):

- [Hetzner](https://www.hetzner.com)
- [Vultr](https://www.vultr.com)
- [DigitalOcean](https://www.digitalocean.com)
- [Heroku](https://www.heroku.com)

## Some Closing Thoughts

While the *pay-as-you-go* model is a good way for cloud providers to increase their revenues, it's a really bad deal for any business that doesn't have "unlimited" funding or huge tax credits, and that can go bankrupt due to a `for` loop.
