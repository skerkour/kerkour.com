+++
date = 2022-08-25T06:30:00Z
title = "Fighting too many open Tabs"
type = "post"
tags = ["minimalism", "productivity"]
authors = ["Sylvain Kerkour"]
url = "/too-many-tabs"

[extra]
lang = "en"
+++

How many browser tabs do you have open? 50? 100? Why?

Actually, this is such a common problem that Safari, Apple's Web Browser, recently introduced Tab Groups to try to manage the open tabs pandemic.

![Safari Tab Groups](https://kerkour.com/2022/too-many-tabs/safari_tab_groups.png)


If so, then you are doing something wrong.

I don't mean that your reasons for having these tabs open are bad. I mean that you are cluttering your digital space and that, over time, it reduces your ability to focus on what really matters. If you keep these tabs open, it's because they contain some information that you find valuable, and you think that it's a good idea to keep it under the elbow.

Instead of keeping a tab open or adding the page to my bookmarks, I create a note in my knowledge base with the most important points extracted (in the case where the website goes down, which happens more often than expected), some tags to organize the note, and the link to the page.


A **[knowledge base](https://kerkour.com/continuous-learning/knowledge-base)** is a central repository for everything you have ever read or watched and you find valuable, organized in a way such as it can be easily searched later.

Thus, instead of keeping these distracting and chaotic tabs open, you create a new note in your knowledge base for each tab, extracting the most important part of the web page (in the case that the website goes down) and adding a link to the actual web page.


When I know that I have already read something and want to remember the details, I just have to search my knowledge base, instead of my notes, my open tabs, my bookmarks and so on...

Here is an example of a page from my own knowledge base that should have been a new tab if I was disorganized:

```markdown
# Anycast vs GeoIP CDN

#cdn #programming #bookmark #anycast #devops

url: https://bunny.net/blog/anycast-vs-geoip-routing/

## Anycast

> First, let's have a look at anycast. Anycast is a network and routing method in which multiple servers will announce the same IP address. When a computer connects to such an address, it will be automatically routed to the server with the closest logical path.


### Pros

* Slightly better performance
* Better redundancy
* Easier DDoS mitigation

### Cons

* Difficult to maintain
* Requires constant optimisations
* Harder to tweak routes



## GeoIP

On the other hand, GeoIP works at the DNS level to direct the user to the closest server.

### Pros

* Advanced load balancing
* Easier to set up and maintain
* Lower cost
* Simple traffic management

### Cons

* Less accurate routing
* Slightly worse performance
* Harder to disperse DDoS attacks
* Slower downtime mitigation

```
