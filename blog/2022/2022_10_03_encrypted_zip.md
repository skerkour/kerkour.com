+++
date = 2022-10-03T04:30:00Z
title = "How to create an encrypted zip file with a password"
type = "post"
tags = ["hacking", "privacy", "linux", "cryptography"]
authors = ["Sylvain Kerkour"]
url = "/zip-encrypted-file-password"

[extra]
lang = "en"
+++

You can create an encrypted zip archive with the following command:

```bash
$ zip -er my_archive.zip file1 folder1 ....
```


Keep in mind that when using the `zip` command, only the content of the zipped files is encrypted, the archive still leaks metadata such as the name of the files.

An alternative is to first create your zip archive with `zip -r file1 ...` and [then use OpenSSL to encrypt the entire ZIP archive](https://kerkour.com/openssl-encrypt-decrypt-file).

## macOS

Unfortunately, macOS comes with an [old and insecure version of zip](https://www.quora.com/How-secure-are-encrypted-Zip-files):

```bash
zip --version
Copyright (c) 1990-2008 Info-ZIP - Type 'zip "-L"' for software license.
This is Zip 3.0 (July 5th 2008), by Info-ZIP.
```

It means that the zip files encrypted on macOS can easily be cracked with [pkcrack](https://github.com/keyunluo/pkcrack) or [johnTheRipper](https://www.openwall.com/john/).

This is why I recommend to [use OpenSSL instead to encrypt your zip file](https://kerkour.com/openssl-encrypt-decrypt-file).
