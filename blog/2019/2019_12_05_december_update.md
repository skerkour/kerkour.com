+++
date = 2019-12-05T10:00:00+01:00
title = "Bloom December update: Native applications are coming"
tags = ["bloom", "rust", "flutter", "open-source"]
type = "post"
author = "Sylvain Kerkour"
authors = ["Sylvain Kerkour"]
url = "/2019-december-update"

[extra]
lang = "en"
+++


It's been exactly 6 months since <a href="/bloom-a-free-and-open-source-google" target="_blank" rel="noopener">Bloom launched</a>.
6 incredible months with a lot of sweat and some sleepless nights.

But something even more incredible is coming during the next month: The launch of Bloom's native applications,
featuring among other things end to end encryption and offline support.


## The past few months

First of all I want to apologize for the lack of public communication the past few months.

*Empowering people with open technologies* requires a lot of cross domain competencies, whether it be
finance, project management, marketing, software development, cryptography and so on...

Being the sole person working on Bloom full time (Marina no longers have enough free time) means that I need to acquire myself all this knowledge, and, more importantly,
how to apply this knowledge to create an awesome product. It takes a lot of time, efforts and focus.

But the largest part is behind us and you can expect more openness in the coming months.

## From cloud to local first

*Cloud apps like Google Docs and Trello are popular because they enable real-time collaboration with colleagues, and they make it easy for us to access our work from all of our devices. However, by centralizing data storage on servers, cloud apps also take away ownership and agency from users. If a service shuts down, the software stops functioning, and data created with that software is lost.* from <a href="https://www.inkandswitch.com/local-first.html" target="_blank" rel="noopener">InkAndSwitch</a>.

But first of all, cloud is a business model. Cloud's first objective is not to empower users or ease their lives, but to
create recurring revenue streams for enterprises (and investors).

What would happen to users' data if the
enterprise go bakrupt? If the enteprise decides to kill the product because it's not enough profitable?
If the enterprise's cloud get hacked? If an embargo is promulgated? If internet is cut for some days/weeks?

Cloud induces dependency which is the exact opposite of empowerement.


### Native applications

The first step is to remove the cloud dependency. It means that Bloom should no longer be accessible through a web browser but as an
installed application that can work without internet.
It means that Bloom will be usable even if you go far away from internet for 3 months.

It also means that **the web application (<a href="https://legacy.bloom.sh" target="_blank" rel="noopener">https://legacy.bloom.sh) is deprecated and all data will be deleted on February 29, 2020**. We provide a program to automatically backup data from *https://legacy.bloom.sh*: [https://github.com/skerkour/bloom](https://github.com/skerkour/bloom).

### Cryptography

For the better or the worse, computers made copying data virtually free. It's inevitable.
Whether it be an eBook, an image or a text file, whether the data is public or private. In the long run,
any bit of data will either become public or will be lost.

But the asymmetry between those who store and proceed the data (*cloud providers*), and those who produce and use
the data (*people*) create great power imbalance.

This is why, as of today, cryptography is <a href="https://en.wikipedia.org/wiki/Permanent_Record_(autobiography)" target="_blank" rel="noopener">the only mean to equilibrate power between those 2 parties</a>.


With cryptography, **You** choose to whom you give access to **What**. **You** choose **Who** can
read your messages, not any random Facebook employee.

<div class="center">
  <img src="cia_triad.jpg" height="500"/>

  <a href="https://en.wikipedia.org/wiki/Information_security#Basic_principles" target="_blank" rel="noopener">The C.I.A triad</a>
</div>

Cryptography gives you confidentiality and integrity for your data,
and paying for Bloom gives you availability: high reliability backup,
cross-devices synchronization, always online servers...


### Collaboration

Finally, humans operate in terms of networks (a.k.a communities, groups) and this is why we have included
the *groups* feature, where you will be able to share a drive space, notes, tasks and so on with your peers.


## Summary

* **The web application (<a href="https://legacy.bloom.sh" target="_blank" rel="noopener">https://legacy.bloom.sh) is deprecated and all data will be deleted on February 29, 2020**. We provide a program to automatically backup data from *https://legacy.bloom.sh*: [https://github.com/skerkour/bloom](https://github.com/skerkour/bloom).
* Native applications are coming in January, featuring end to end encryption and offline support among other improvements
* **You will need to create a new account on the native application**
* New Mastodon account: <a href="https://mastodon.social/@42bloom" target="_blank" rel="noopener">@42bloom@mastodon.social</a>
* Paying offers (more Drive space, more Bitflow downloads...) will be available with the native applications
* Next months will be dedicated to secure revenues and grow the core team

<br />
<br />
Have a great day ✌️
