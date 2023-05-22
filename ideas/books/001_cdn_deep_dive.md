+++
date = 2022-01-11T06:00:00Z
title = "2022_01_11_cdn_deep_dive"
type = "post"
tags = []
authors = ["Sylvain Kerkour"]
url = "/2022_01_11_cdn_deep_dive"
draft = true

[extra]
lang = "en"

comment ="""
"""
+++


CDN deep dive: How to boost the rperformance and secure your web application in a few lines of code

https://haydenjames.io/best-cdn-providers/

https://news.ycombinator.com/item?id=23894827


## Origin location

https://tools.keycdn.com/ping


# What are CDNs


Cdn deep dives

Mitm and static

I've tested only the following ones because they are the simpler to setup

Anycast vs geoip


https://github.com/leandromoreira/cdn-up-and-running

https://www.learncsdesign.com/an-overview-of-distributed-caching/


CDN pour sites static vs CDN pour app dynamiques
dans le premier cas, on aura tendance a purger le CDN a chaque deploy, alors que dans le second cas, on aura tendance a avoi rdes Cache-control header differents en fonctions des parties du sites


donnes des exemples de cache control
- site static
- site dynamic (rails)
- SPA wiht JSON API
