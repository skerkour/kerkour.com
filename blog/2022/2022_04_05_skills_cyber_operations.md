+++
date = 2022-04-05T02:00:00Z
title = "The 5 profiles of cyber operators"
type = "post"
tags = ["hacking"]
authors = ["Sylvain Kerkour"]
url = "/profiles-cyberattacks"

[extra]
lang = "en"

comment ="""
"""
+++



The profile of attackers is highly diversified. From lone wolves to teams of hackers, developers to analysts, there is definitely not a common profile that fits them all. However, in this section, I will try to portray which profiles should be part of a team conducting offensive operations.


> This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where you'll learn Rust, offensive security and cryptography.



### The hacker

The term hacker is controversial: mainstream media use it to describe criminals while tech people use it to describe passionate or hobbyists tinkering with tech. In our context, we will use it to describe the person with advanced offensive skills and whose role is to perform reconnaissance and exploitation of the targets.


### The exploit writer

The exploit writers are often developers with a deep understanding of security. Their role is to craft the weapons used by their teams to break into their targets' networks and machines.

Exploit development is also known as "**weaponization**".

Entire companies are operating in the grey waters of exploits trading, such as Vupen or Zerodium. They often don't find the exploits themselves but buy them from third-party hackers and find buyers (such as government agencies or malware developers).


### The developer

The role of the developer is to build custom tools (credential dumpers, proxies...) and implants used during the attack. Indeed, using publicly available, pre-made tools vastly increase the risk of being detected.

These are the skills we will learn and practice in the next chapters.


### The system administrator

Once the initial compromise is performed, the role of the system administrator is to operate and secure the infrastructure used by attackers. Their knowledge can also be used during the exploitation and lateral movements phases.


### The analyst

In all kinds of attacks, domain knowledge is required to interpret the findings and prioritize targets. This is the role of the analyst, either to provide deep knowledge about what specifically to target or to make sense of the exfiltrated data.

