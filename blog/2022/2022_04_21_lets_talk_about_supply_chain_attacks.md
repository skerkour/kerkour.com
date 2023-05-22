+++
date = 2022-04-21T02:00:00Z
title = "Let's talk about supply chain attacks and backdoored dependencies"
type = "post"
tags = ["hacking"]
authors = ["Sylvain Kerkour"]
url = "/supply-chain-attacks-and-backdoored-dependencies"

[extra]
lang = "en"

comment ="""

1. Buy expired NPM maintainer email domains.
2. Re-create maintainer emails
3. Take over packages
4. Submit legitimate security patches that include package.json version bumps to malicious dependency you pushed
5. Enjoy world domination.
"""
+++

![ostrich](https://kerkour.com/2022/supply-chain-attacks-and-backdoored-dependencies/ostrich.jpg)
*The software industry when we hear the word "security"*



There is no [one month](https://github.com/advisories/GHSA-97m3-w2cp-4xx6) without some [popular](https://www.mandiant.com/resources/evasive-attacker-leverages-solarwinds-supply-chain-compromises-with-sunburst-backdoor) [dependencies](https://www.proofpoint.com/us/blog/threat-insight/serpent-no-swiping-new-backdoor-targets-french-entities-unique-attack-chain) [found to be](https://www.bleepingcomputer.com/news/security/over-90-wordpress-themes-plugins-backdoored-in-supply-chain-attack/) [compromised](https://github.com/dominictarr/event-stream/issues/116) or [backdoored](https://jfrog.com/blog/malicious-pypi-packages-stealing-credit-cards-injecting-code/).

I've already written [about how easy it is to insert a stealth backdoor](https://kerkour.com/rust-crate-backdoor) in a software package, so today we are going to see how attackers get write access to the packages in the first place.

## Supply chain attacks in practice

What makes supply chain attacks particularly attractive is that they are cheap and easy to execute as they (most of the time) don't require exploits, the attack surface is huge, they can have a large reach, with packages downloaded 1M+ times a week for example, and because they allow remote code execution on developers' machines, the exfiltrated data and credentials can be used to reach even more targets which make them a good way to spread a worm.


### Developer gone rogue

We are all human and have many conflicting thoughts. Thus any package maintainer can go rogue for many reasons: corruption, ideology...

There was a recent case with an NPM package with roughly 1 000 000 weekly downloads: [node-ipc](https://github.com/advisories/GHSA-97m3-w2cp-4xx6) where the developer embedded malware in the library to protest against the Russian-Ukrainian war and to sabotage computers connected to internet with a Russian IP address.

### Compromised CI/CD infrastructure

Most companies do not monitor and protect their CI/CD infrastructure the same way that they defend their production systems. As a result, a lot of CI/CD pipelines rely on [outdated software](https://www.cvedetails.com/vulnerability-list/vendor_id-15865/Jenkins.html) with [vulnerabilities](https://www.cvedetails.com/vulnerability-list/vendor_id-13074/product_id-26968/Gitlab-Gitlab.html).

Once compromised, the CI/CD infrastructure can not only be used to backdoor the projects that go through it, but also to steal environment variables and perform lateral movements.

You can read more about compromised CI/CD pipelines in [this excellent post](https://research.nccgroup.com/2022/01/13/10-real-world-stories-of-how-weve-compromised-ci-cd-pipelines/) by nccgroup.


### Typosquatting


By naming a package in a very similar way to a popular one, we can expect that a non-zero number of developers will make a typo in the name, either when searching on the repository's searchbar or when installing the package. `request` instead of `requests` for example.



Don't be over-confident that it can never happen to you: [here is an example](https://github.com/axios/axios/pull/4594#pullrequestreview-952714598) where the typo came from the auto-completion feature of the code editor.

### Dependency confusion

Dependency confusion attacks rely on the fact that some package repositories silently serve a public package with the same name as a private package. So when a private, internal package is requested, a public package, controlled by an attacker,  is served instead, which leads to remote code execution.

You can read more about this family of sneaky attacks in [this excellent write-up](https://medium.com/@alex.birsan/dependency-confusion-4a5d60fec610).


### Compromised credentials

Whether it be by phishing, database hacks, or malware, it's more and more common to see sales of compromised credentials on hacking forums, which then can then be used to get write access to software packages.

How to prevent this kind of attack? [MFA, MFA, MFA](https://en.wikipedia.org/wiki/Multi-factor_authentication)!


### Compromised Publish Keys

More worrisome is the compromission of Publish Keys/Tokens. In contrary to compromised usernames/passwords, there is no Multi-Factor Authentication available for tokens.

Also, they are very rarely rotated in the real world.


### Compromised Cookies


In the same way that tokens can be stolen, authentication cookies can also be stolen, sold on hacking forums, and then reused by attackers to impersonate a developer who has access to a software repository.


One recent example of such a technique was [EA Games' hack](https://www.vice.com/en/article/7kvkqb/how-ea-games-was-hacked-slack), where hackers bought stolen cookies to impersonate an employee on a Slack workspace, and then social engineer system administrators to get more access to EA's systems.


Unfortunately, from a user's perspective, it's very hard to protect against this kind of attack, and it's up to the service providers to implement short-lived authentication tokens.


### Compromised Git servers

Another possibility for attackers is to directly compromise the servers hosting the source code of the packages. While rarer than other attacks, due to the prevalence of Cloud-hosted Git services, I've myself found many, many misconfigured or [vulnerable](https://www.cvedetails.com/vulnerability-list/vendor_id-13074/product_id-26968/Gitlab-Gitlab.html) GitLab instances.


One notable example is the [hack of PHP's Git server](https://news-web.php.net/php.internals/113838), where attackers were able to push 2 malicious commits.


### Expired email address domain name

Let's imagine that the developer of the `very-popular` package uses their own domain for their email address: `john@family-name.com` for example.

They may forget to renew their domain name, which may lead someone else to buy it and set up an email server for this domain. By doing that, an attacker will be able to take over all the published packages.

It [recently happened](https://twitter.com/vxunderground/status/1523982714172547073) to the [`foreach`](https://www.npmjs.com/package/foreach) NPM package with more than 5M weekly downloads.

## Some Closing Thoughts


Are supply chain attacks and backdoor unstoppable?

I don't think so, but I think that the current model of centralized package managers needs to end. Centralized package repositories add layers of obfuscation and complexity, which is only good for attackers.

I think that a better model for package management is decentralized distribution with centralizedd search.

This is why I think that today, [Go has the best approach to supply chain security](https://go.dev/blog/supply-chain): Git-based dependency distribution, which enables easier auditing, a centralized database of modules' content so that everybody get the same code for a given `module@version`, and no build-time scripts.

While we will never be able to stop the compromise of 3rd party libraries, we can do our best to ease the detection.
