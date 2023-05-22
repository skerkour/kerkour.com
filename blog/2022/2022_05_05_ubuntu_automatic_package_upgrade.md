+++
date = 2022-05-05T02:00:00Z
title = "How to enable automatic updates on Ubuntu"
type = "post"
tags = ["programming", "linux", "tutorial", "devops"]
authors = ["Sylvain Kerkour"]
url = "/ubuntu-linux-automatic-updates-with-unattended-upgrades"

[extra]
lang = "en"

comment ="""
https://sudoedit.com/ubuntu-live-patching/
https://linoxide.com/enable-automatic-updates-on-ubuntu-20-04/
"""
+++

Life is too short to manually upgrade the packages of your machine twice a week, so here is how to automate the software updates of an Ubuntu server (20.04 or 22.04).

First, you need to install `unattended-upgrades`:

```bash
$ sudo apt install unattended-upgrades
```

Then configure it:
```
$ sudo dpkg-reconfigure -plow unattended-upgrades
```

and select `yes` on the dialog.

Then, you need to enable automatic updates on a daily basis:
```bash
$ vim /etc/apt/apt.conf.d/10periodic
```

```
APT::Periodic::Update-Package-Lists "1";
APT::Periodic::Download-Upgradeable-Packages "1";
APT::Periodic::AutocleanInterval "7";
APT::Periodic::Unattended-Upgrade "1";
```


Finally, you may want to disable kernel updates that require a reboot:

```bash
$ sudo vim /etc/apt/apt.conf.d/50unattended-upgrades
```
```
Unattended-Upgrade::Package-Blacklist {
   // The following matches all packages starting with linux-
   "linux-";
};

Unattended-Upgrade::Automatic-Reboot "false";
```
