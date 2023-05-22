+++
date = 2022-05-06T02:00:00Z
title = "Patching the Linux kernel without reboot with Ubuntu livepatch"
type = "post"
tags = ["programming", "linux", "tutorial", "devops"]
authors = ["Sylvain Kerkour"]
url = "/linux-update-kernel-without-reboot-with-ubuntu-livepatch"

[extra]
lang = "en"

comment ="""
https://sudoedit.com/ubuntu-live-patching/
"""
+++


When you want to maximize the uptime of your servers, you need a way to update the kernel to fix security vulnerabilities without having to reboot the machines.

For that, Canonical provides [livepatch](https://ubuntu.com/security/livepatch): a way to apply live security patches to the Ubuntu's kernel, without the need to reboot.

And I have good news: Canonical provides this service for free for up to 3 machines (desktops or servers)!

The first step is to get your Ubuntu Advantage token at the following address: [https://ubuntu.com/advantage](https://ubuntu.com/advantage)

Then on your machine:

```bash
$ sudo ua attach [YOUR TOKEN]
$ sudo ua enable livepatch
$ ua status
```


Take a look at [https://ubuntu.com/livepatch](https://ubuntu.com/livepatch) for more information.
