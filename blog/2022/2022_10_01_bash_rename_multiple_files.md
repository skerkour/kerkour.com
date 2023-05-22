+++
date = 2022-10-01T06:30:00Z
title = "How to rename multiple files in bash and remove a given prefix or suffix"
type = "post"
tags = ["newsletter", "linux", "bash"]
authors = ["Sylvain Kerkour"]
url = "/bash-rename-multiple-files-prefix-suffix"

[extra]
lang = "en"
+++

You may want to rename multiple files in Bash to remove a given prefix or suffix, here is how to do.


## Remove a prefix in bash

To rename all your files like `myPrefix - myfile.mp3` to `myfile.mp3`.

```bash
for name in "myPrefix -"*;
do
    newname="$(echo "$name" | cut -d' ' -f3-)"
    mv "$name"  "$newname"
done
```


## Remove a suffix in bash without removing the extension

To rename all your files like `myfile_256kbps.mp3` to `myfile.mp3`.

```bash
for name in *_256kbps.mp3;
do
    newname="$(echo "$name" | rev | cut -d'_' -f2- | rev)".mp3
    mv "$name"  "$newname"
done
```

## Change the extension in bash

To convert all your files like `my_book.epub` to `my_book.mobi`.


```bash
for f in ./*.epub; do
  ebook-convert "$f" "${f%.epub}.mobi"
done
```
