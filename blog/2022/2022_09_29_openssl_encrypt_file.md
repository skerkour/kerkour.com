+++
date = 2022-09-29T06:30:00Z
title = "How to use OpenSSL to encrypt/decrypt files"
type = "post"
tags = ["hacking", "privacy", "linux", "cryptography"]
authors = ["Sylvain Kerkour"]
url = "/openssl-encrypt-decrypt-file"

[extra]
lang = "en"
+++

Encrypt a file with OpenSSL:

```bash
$ openssl enc -aes-256-gcm -salt -pbkdf2 -iter 100000 -in my_file.jpg -out my_file.jpg.enc
```

Decrypt a file with OpenSSL:

```bash
$ openssl enc -d -aes-256-gcm -salt -pbkdf2 -iter 100000 -in my_file.jpg.enc -out my_file2.jpg
```

## macOS

Unfortunately, on macOS LibreSSL is used instead of OpenSSL. You can see it by running the following command in your terminal:

```bash
$ openssl version
LibreSSL 2.8.3
```

Thus, the newer and more secure [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) algorithm is not available to turn your password into a secure encryption key.

Instead you can use the following command to encrypt your file:

```bash
$ openssl enc -aes-256-gcm -salt -md sha512 -in my_file.jpg -out my_file.jpg.enc
```


And this one to decrypt the encrypted file:

```bash
$ openssl enc -d -aes-256-gcm -salt -md sha512 -in my_file.jpg.enc -out my_file2.jpg
```
