+++
date = 2023-03-18T07:00:00Z
title = "How to deploy a self-hosted Docker registry"
type = "post"
tags = ["devops", "programming", "docker"]
authors = ["Sylvain Kerkour"]
url = "/deploy-self-hosted-docker-registry"
draft = true

[extra]
lang = "en"
+++

<!--
https://chat.openai.com/chat/ec66325b-9805-4be0-b0cb-9103f622ffc2
https://docs.docker.com/registry/configuration/#auth
https://docs.docker.com/registry/spec/auth/token/
https://docs.docker.com/registry/recipes/nginx/
https://httptoolkit.com/blog/docker-image-registry-facade/
-->

Docker, the company behind the ubiquitous tool with the same name announced [this week that they will delete "organisation" accounts](https://www.docker.com/blog/we-apologize-we-did-a-terrible-job-announcing-the-end-of-docker-free-teams/) that were free until now if they do not upgrade to a paid team plan.

While I'm not entitled to get free services from a company, these kinds of bait and switch always leave a bitter taste when you participate in making a platform valuable and they destroy the trust relationship when they have reached their targeted growth metrics.

Fortunately, self-hosting your own Docke registry is pretty easy, so here is how to do it. In the second part of this blog post we will see how to store the data in an S3-compatible bucket instead of the local disk.

## Using S3 for storage
