+++
date = 2022-03-09T02:00:00Z
title = "Hacking Stories #3 - The Puppet Master"
type = "post"
tags = ["hacking-stories", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/hacking-stories/puppet-master"


[extra]
lang = "en"

comment ="""
Bob vietn est un developpeur....
Il travaille dans une boite qui produit une extension JS

il se fait virer

plsu d'argent

sa vie tourne au cauchemar

Il decide de se venger.

Il hack son ancienne entreprise en backdoorant une dependendence

cela transforme tous les users en botnet

Qu'il met en vente en crypto








--------------------

bob dev une extension de browser pour le plaisir

on lui a propose plusieurs fois d'introduire du code.... contre de l'argent, il a toujorus refuse.

Mais lorsque que la guerre eclata...


Il les utilise pour la guerre en ukraine:
- Ddos
- steal cookies from russian computers



4 parties:

extensions
proposer contre de l'argent
guerre qui eclate
utilisation de l'extension pour faire un botnet

"""
+++

{{< hacking_stories_disclaimer >}}


Andrei lives a calm and quiet life in the countryside of a small country. The day he is working remotely as a developer for a company on the other side of the planet. The night, he develops an open-source browser extension that allows people to quickly share screenshots and screen recordings with their friends and co-workers.

Due to the recent pandemic and the advent of remote work, his extension saw a huge spike of users, now counting in the millions.

As his extension is now very popular, he received countless offers from dubious companies to include ads, or questionable snippets of code. Because he is himself the first user of his extension, he never accepted these offers, knowing how it would make him lose the trust of his users in a wink, tarnish the hard-earned brand of his browser extension, and, more importantly, how it would pose a huge security threat to his users.

Because Andrei really enjoys his day job, he doesn't want to turn his extension into a business and has to find a way to reduce the costs of hosting the backend and find a way to reduce the time it takes him to maintain the infrastructure. He decided to use [heroku for the infrastructure](https://kerkour.com/deploy-rust-on-heroku-with-docker), so he no longer has to maintain servers himself. But, Heroku is very expensive, and with the current backend in Node.js, which is very inefficient, the costs were way too high for a simple side-project.

For these reasons, he decided to rewrite the backend in Rust and also use Rust in the frontend with WebAssembly to do the images encoding client-side. It was a good occasion for him to [learn this new language](https://kerkour.com/black-hat-rust), which is [Stack Overflow's most loved language](https://insights.stackoverflow.com/survey/2021) for many years.

One day, Andrei's world collapsed. The neighboring country just got invaded. The situation escalated so fast that most people didn't really see it coming.

Now that a real war is at the border of his own country, threatening his family's and his own lives, he felt that he had to act.

He first tried to hack websites of the country of the attacker, with SQL and XSS injections, like in movies. But he quickly realized that the impact was too low.

After reading an article [about supply chain security](https://kerkour.com/rust-crate-backdoor) he thought: "what if I decide to backdoor my own browser extension?". Indeed, he has full control over the source code, and, more importantly, of the CI (Continuous Integration) infrastructure.

To remain as stealthy as possible, he decided to inject his backdoor at build time.

A backdoor that turned his web browser extension into a botnet composed of millions of agents across the world.

But a backdoor to do what?

The first feature he implemented was simple HTTP [DDoS](https://en.wikipedia.org/wiki/Denial-of-service_attack). With millions of infected browsers, he could help other hacktivists DDoSing the infrastructure of the attacker.

The second feature he implemented was a Cookies stealer: all the browsers from the attacking country are now sending **all** their cookies to a server controlled by Andrei, so that they can be reused to impersonate the victims.

The third feature was network scanning on the infected machines, in order to scan internal networks the machines are connected to, and which networked services are running.

All this data is then anonymously dumped on hacker forums and chat channels, organizing cyber operations against the attacker.

To keep the backdoor as obfuscated as possible, he decided to use a WebAssembly backdoor, built in Rust.

He only needed to use the [web-sys](https://crates.io/crates/web-sys) crate to interact with the browsers' features. Everything else was implemented from scratch in a few days.

In order to keep low profile, Andrei decided to include 2 anti-detection measures.

First, the communication between the botnet and the server are [end-to-end encrypted](https://kerkour.com/signatures-modern-end-to-end-encryption) using `X25519` key exchange and `XChaCha20-Poly1305`. Nothing fancy.

The second countermeasure is that an infected browser never sends the exfiltrated data to the server in plain JSON. Instead, the encrypted data is sent back to the server hidden in screenshots, using [steganography](https://en.wikipedia.org/wiki/Steganography).


It's been 1 week now, and Andrei has already exfiltrated Gigabytes of very sensitive data, that is then used to target the systems of the attacker.

But he is not sure to continue, first not to be unmasked, but, more importantly, because war makes him very uncomfortable, and he finally doesn't want to be responsible for the death of many people who were convinced to join a war that they have nothing to do about it because of propaganda.

Until the day when the attackers extend their operations to other countries...


**Want to learn Rust, Cybersecurity and Cryptography? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in  Rust.**
