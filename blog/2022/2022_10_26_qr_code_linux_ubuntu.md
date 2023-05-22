+++
date = 2022-10-26T06:30:00Z
title = "Scanning and generating QR Codes on Ubuntu Linux with the command line"
type = "post"
tags = ["linux", "hacking", "ubuntu"]
authors = ["Sylvain Kerkour"]
url = "/scan-and-generate-qr-code-ubuntu-linux-command-line"

[extra]
lang = "en"
+++

## Scanning QR Codes on Linux with the command line

With `qrcode.jpg` the image file with the QR Code to scan.

```bash
$ sudo apt install zbar-tools
$ zbarimg qrcode.jpg
```

## Generating QR Codes on Linux with the command Line

To generate a QR Code in `qrcode.jpg`.

```bash
$ sudo apt install qrencode
$ qrencode -o qrcode.jpg "my data"
```
