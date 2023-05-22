+++
date = 2022-05-18T02:00:00Z
title = "Bitsquatting attacks and exploitation with Rust"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/bitsquatting-attack-generator-in-rust"

[extra]
lang = "en"

comment ="""
"""
+++


I personally find bitsquatting attacks mind-blowing!

The idea is that computers suffer from memory errors where one or more bits are corrupted, they are different than their expected value. It can come from electromagnetic interference or [cosmic rays](https://www.johndcook.com/blog/2019/05/20/cosmic-rays-flipping-bits/) (!).

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


A bit that is expected to be `0`, may flips to `1`, and vice versa.


![Bit flip](https://kerkour.com/2022/bitsquatting-generator-in-rust/ch09_bitflip.svg)



In this example, if attackers control `acc.com`, they may receive originally destined for `abc.com` **without any human error**!


Here is a small program to generate all the "bitshifted" and valid alternatives of a given domain:
**[ch_09/dnsquat/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/dnsquat/src/main.rs)**
```rust
use std::env;

fn bitflip(charac: u8, pos: u8) -> u8 {
    let shiftval = 1 << pos;
    charac ^ shiftval
}

fn is_valid(charac: char) -> bool {
    charac.is_ascii_alphanumeric() || charac == '-'
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage: dnsquat domain .com");
        return;
    }

    let name = args[1].to_lowercase();
    let tld = args[2].to_lowercase();

    for i in 0..name.len() {
        let charac = name.as_bytes()[i];
        for bit in 0..8 {
            let bitflipped = bitflip(charac.into(), bit);
            if is_valid(bitflipped as char)
                && bitflipped.to_ascii_lowercase() != charac.to_ascii_lowercase()
            {
                let mut bitsquatting_candidat = name.as_bytes()[..i].to_vec();
                bitsquatting_candidat.push(bitflipped);
                bitsquatting_candidat.append(&mut name.as_bytes()[i + 1..].to_vec());

                println!(
                    "{}{}",
                    String::from_utf8(bitsquatting_candidat).unwrap(),
                    tld
                );
            }
        }
    }
}
```

```bash
$ cargo run -- domain .com
eomain.com
fomain.com
lomain.com
tomain.com
dnmain.com
dmmain.com
dkmain.com
dgmain.com
dolain.com
dooain.com
doiain.com
doeain.com
do-ain.com
domcin.com
domein.com
domiin.com
domqin.com
domahn.com
domakn.com
domamn.com
domaan.com
domayn.com
domaio.com
domail.com
domaij.com
domaif.com
```
