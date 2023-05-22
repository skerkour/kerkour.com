+++
date = 2021-04-08T14:00:00Z
title = "42 Companies using Rust in production"
type = "post"
tags = ["rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/rust-in-production-2021"

[extra]
lang = "en"

comment ="""
resources

https://bionic.fullstory.com/rust-at-fullstory-part-2/
https://www.ditto.live/blog/posts

https://news.ycombinator.com/item?id=23008399

https://news.ycombinator.com/item?id=26263518

https://news.ycombinator.com/item?id=26647981

https://security.googleblog.com/2021/04/rust-in-android-platform.html

https://www.reddit.com/r/rust/comments/lfhs3z/trusted_programming_our_rust_mission_at_huawei/

https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine
"""
+++

[A lot of people want to learn Rust](https://insights.stackoverflow.com/survey/2021#technology-most-loved-dreaded-and-wanted) but are afraid that the language or the ecosystem is not production-ready yet, or that they may not find a job.

Be reassured, Rust is already used in production, from small shops to the largest companies in the world, serving billions of transactions daily, if not hourly. Whether it be for its reliability, productivity, speed, or safety, Rust is a language of choice, **today**.

The following list is **not** exhaustive, I picked some of my 42 favorite stories about using Rust in production now (or very soon).


**[1Password](https://serokell.io/blog/rust-in-production-1password)**: *"We’ve been using Rust in production at 1Password for a few years now. Our Windows team was the frontrunner on this effort to the point where about 70% of 1Password 7 for Windows is written in Rust. We also ported the 1Password Brain – the engine that powers our browser filling logic – from Go to Rust at the end of 2019 so that we could take advantage of the speed and performance of deploying Rust to WebAssembly in our browser extension."*

**[Android](https://security.googleblog.com/2021/04/rust-in-android-platform.html)**: *"For the past 18 months we have been adding Rust support to the Android Open Source Project, and we have a few early adopter projects that we will be sharing in the coming months. Scaling this to more of the OS is a multi-year project."*

**[Astropad](https://blog.astropad.com/why-rust/)**: *"With Rust, we’ll have a high-performance and portable platform that we can easily run on Mac, iOS, Linux, Android, and Windows. Not only will this drastically expand our potential market size, but we also see many interesting new uses for our LIQUID technology that we’ll be able to pursue with our Rust based platform. We’re confident that we’ll finish our Rust journey with stronger code, better products, and an optimistic outlook for Astropad’s future."*

**[AWS](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)**: *"Here at AWS, we love Rust, too, because it helps AWS write highly performant, safe infrastructure-level networking and other systems software. Amazon’s first notable product built with Rust, Firecracker, launched publicly in 2018 and provides the open source virtualization technology that powers AWS Lambda and other serverless offerings. But we also use Rust to deliver services such as Amazon Simple Storage Service (Amazon S3), Amazon Elastic Compute Cloud (Amazon EC2), Amazon CloudFront, Amazon Route 53, and more. Recently we launched Bottlerocket, a Linux-based container operating system written in Rust."*

**[Bloom](https://github.com/skerkour/bloom)**: *"As an indie and boostraped business, I need Bloom to be as reliable as possible, so I can enjoy my nights and holidays. Rust is a perfect fit thanks to its awesome compiler that catches most of the bugs at compile time.*"

**[Cloudflare](https://blog.cloudflare.com/tag/rust/)**: *"As our experience with Rust grew, it showed advantages on two other axes: as a language with strong memory safety it was a good choice for processing at the edge and as a language that had tremendous enthusiasm it became one that became popular for de novo components."*

**[Coursera](https://medium.com/coursera-engineering/rust-docker-in-production-coursera-f7841d88e6ed)**: *"We efficiently, reliably, and securely grade assignment submissions inside hardened Docker containers. Although we offload the cluster scheduling to Amazon EC2 Container Service (ECS), to orchestrate all of the moving pieces we have a number of additional programs that work in concert, some of them written in Rust."*

**[Cultivate](https://cultivatehq.com/posts/how-we-built-a-visual-studio-code-extension-for-iot-prototyping/)**: *"Rust allowed the experienced developers to move fast and develop features quickly with less errors, and allowed those exploring systems development for the first time to move forward with confidence, curiosity and protection from the "gotchas" of C/C++."*

**[Crisp](https://github.com/valeriansaliou/vigil)**: *"Vigil is an open-source, self-hosted status page, monitoring and alerting system. It is built in Rust, which makes it reliable, lightweight and crash-free (at least, in theory)."*

**[Discord](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f?gi=e5dd22290878)**: *"When starting a new project or software component, we consider using Rust. Of course, we only use it where it makes sense. Along with performance, Rust has many advantages for an engineering team. For example, its type safety and borrow checker make it very easy to refactor code as product requirements change or new learnings about the language are discovered. Also, the ecosystem and tooling are excellent and have a significant amount of momentum behind them."*

**[Ditto](https://www.ditto.live/blog/posts/introducing-safer-ffi)**: *"When we started to build Ditto as a cross-platform SDK, we understood that it was untenable to create a specific port for each popular programming language. Instead, we opted to build the vast majority of shared code in Rust. Rust bought us a lot of features such as being easier to read, highly performant, and includes a modern build system and package manager."*

**[Dropbox](https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine)**: *"We wrote Nucleus in Rust! Rust has been a force multiplier for our team, and betting on Rust was one of the best decisions we made. More than performance, its ergonomics and focus on correctness has helped us tame sync’s complexity. We can encode complex invariants about our system in the type system and have the compiler check them for us."*

**[Everlane](https://precompile.com/2016/06/23/shipping-forgettable-microservices-with-rust.html)**: *Rust affords us forgettability. This service has been in production for 4 months now. It handles on average 40 requests per second with a 10ms response time. Its memory usage rarely goes above 100MB"*

**[Facebook](https://engineering.fb.com/2021/04/29/developer-tools/rust/)**: *Facebook is embracing Rust, one of the most loved and fastest-growing programming languages available today. In addition to bringing new talent to its Rust team, Facebook has announced that it is officially joining the nonprofit Rust Foundation. Alongside fellow members including Mozilla (the creators of Rust), AWS, Microsoft, and Google, Facebook will be working to sustain and grow the language’s open source ecosystem.*

**[Faraday](https://github.com/faradayio?q=&type=&language=rust&sort=)**

**[Figma](https://www.figma.com/blog/rust-in-production-at-figma/)**: *"While we hit some speed bumps, I want to emphasize that our experience with Rust was very positive overall. It’s an incredibly promising project with a solid core and a healthy community."*

**[Fly.io](https://fly.io/docs/reference/architecture/#proxy)**: *"Every server in our infrastructure runs a Rust-based proxy named fly-proxy. The proxy is responsible for accepting client connections, matching them to customer applications, applying handlers (eg: TLS termination), and backhaul between servers."*

**[fullstory](https://bionic.fullstory.com/rust-at-fullstory-part-1/)**: *"After careful consideration, Rust came out ahead on the criteria we chose. While we admit that some of us were hoping that this would be the end result, it was not a foregone conclusion! We have quite a few engineers with deep knowledge of Go and TypeScript, and we used their experience to make sure that those languages didn’t get short shrift—but in the end, the consensus decision was that Rust was, in fact, the right choice."*


{{< subscribe_form >}}

**[IBM](https://developer.ibm.com/technologies/web-development/articles/why-webassembly-and-rust-together-improve-nodejs-performance/)**: *"A team at IBM achieved incredible performance improvements using WebAssembly and Rust"*

**[Huawei](https://trusted-programming.github.io/2021/02/07/our-rust-mission-at-huawei.html)**: *"A journey towards the vision of Trusted Programming has just begun and we hope to work collaboratively with the Rust community, and the upcoming Rust Foundation, to lead a smooth revolution to the Telecom software industry."*

**[KISIO Digital](https://github.com/CanalTP/mimirsbrunn)**

**[Komodo Platform](https://github.com/KomodoPlatform?q=&type=&language=rust&sort=)**

**[Linkerd](https://linkerd.io/2020/07/23/under-the-hood-of-linkerds-state-of-the-art-rust-proxy-linkerd2-proxy/)**: *"The decision to use Rust came down to several factors. First, a service mesh proxy has some pretty stringent requirements: because it’s deployed as a sidecar on a per-pod basis, it has to have as small a memory and CPU footprint as possible. Because most or all of the application’s network traffic flows through the proxy, it needs to have minimal latency overhead, especially worst-case tail latency. Perhaps most importantly, because the proxy handles application data—potentially including incredibly sensitive data such as financial transactions or personal health—it has to be secure."*

**[Microsoft](https://www.youtube.com/watch?v=NQBVUjdkLAA)**

**[Mozilla](https://hacks.mozilla.org/2016/07/shipping-rust-in-firefox/)**: *"Starting with Firefox 48, Mozilla is shipping its first production Rust code, with more to come!"*

**[npm](https://www.rust-lang.org/static/pdfs/Rust-npm-Whitepaper.pdf)**: *"npm’s first Rust program hasn't caused any alerts in its year and a half in production. "My biggest compliment to Rust is that it's boring," offered Dickinson, "and this is an amazing compliment." The process of deploying the new Rust service was straight- forward, and soon they were able to forget about the Rust service because it caused so few operational issues"*

**[OneSignal](https://onesignal.com/blog/tag/rust/)**: *"Just this month, we crossed the threshold of sending 7 Billion notifications per day, and hit a record of 1.75 million deliveries per second."*

**[Qovery](https://www.qovery.com/blog/why-rust-has-a-bright-future-in-the-cloud)**: *"Rust is gaining momentum with companies realizing its benefits for cloud computing. Dropbox used Rust to rewrite some of its core systems while Mozilla used Rust to build the Firefox browser engine demonstrating its robust benefits. At Qovery, we believe in Rust's capabilities for building the future of the Cloud."*

**[Rapid7](https://blog.rapid7.com/2017/06/01/trusting-agents-with-rust/)**: *"The main speed bump we see for our Rust deployment is how easily the deployment tools on different platforms can accommodate the language and how quickly agent developers can learn the language and develop integrations with the managed runtimes."*

**[Samsung](https://community.smartthings.com/t/hub-firmware-release-notes-17-12-17-13-17-14/83722/376)**: *"We want to give out a big shoutout to the Rust Language 5 core team, Mozilla, and the many contributors to packages in the Rust language ecosystem. We are making use of Rust for the backbone of our new update client and server as well as a few other pieces of software and are looking to continuing to expand our use of the language over time as makes sense."*

**[Sentry](https://blog.sentry.io/2016/10/19/fixing-python-performance-with-rust)**: *"This project has been a tremendous success. It took us very little time to implement, it lowered processing times for our users, and it also will help us scale horizontally. Rust has been the perfect tool for this job because it allowed us to offload an expensive operation into a native library without having to use C or C++, which would not be well suited for a task of this complexity."*

**[Signal](https://github.com/signalapp/ringrtc)**

**[Snips](https://github.com/snipsco?q=&type=&language=rust&sort=)**

**[System76](https://blog.system76.com/post/187072707563/one-of-the-remaining-issues-with-firmware)**: *"Like all of our projects today, it is written in Rust, and adheres to current best practices. The project is configured as a workspace, with the core crate providing a generic library for discovering and managing firmware from multiple firmware services. Both fwupd and system76-firmware are supported."*

**[Threema](https://github.com/threema-ch/push-relay)**

**[Tonari](https://blog.tonari.no/why-we-love-rust)**: *"Experiencing no software-related downtime so far is both a pleasant surprise, and a testament to the safety provided by Rust's guarantees. Rust has also made it easy to write performant code with efficient resource usage - both our CPU and memory usage has been predictable and consistent. Without a garbage collector, we can guarantee consistent latency and frame rates."*

**[Veloren](https://veloren.net)**

**[VS Code](https://github.com/microsoft/vscode-ripgrep)**: Ever noticed how fast is VS Code's search? The reason is that VS Code is using [ripgrep](https://github.com/BurntSushi/ripgrep) to power [its search](https://code.visualstudio.com/updates/v1_11#_text-search-improvements).

**[Wire](https://github.com/wireapp/proteus)**

**[Zenly](https://www.rustjobs.dev/job/zenly-senior-back-end-engineer-rust)**

**[yeslogic](https://github.com/yeslogic?q=&type=&language=rust&sort=)**

**[Clever Cloud](https://www.clever-cloud.com/blog/tag/rust/)**: *"For us, these benefits make a strong case for Rust as a reliable building block for a production platform. This is the piece of code we don't have to worry about, and it will enable others to run safely."*

And many more such as [Scaleway](https://careers.scaleway.com/job/backend-developer-api,-golang,-rust-network-products), [Oxide](https://github.com/oxidecomputer?q=&type=&language=rust&sort=), [Fuchsia](https://fuchsia.dev/fuchsia-src/contribute/contributing_to_netstack3), [MeiliSearch](https://github.com/meilisearch/MeiliSearch), [Vector](https://github.com/timberio/vector), [embark](https://medium.com/embarkstudios/inside-rust-at-embark-b82c06d1d9f4), [Chef](https://github.com/habitat-sh/habitat), [BBC](https://twitter.com/tdp_org/status/1380205061775712257)...

You can also find a [dedicated page on the official Rust website](https://www.rust-lang.org/production/users).
