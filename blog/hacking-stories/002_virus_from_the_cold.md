+++
date = 2021-12-22T10:00:00Z
title = "Hacking Stories #2 - The virus that came in from the cold"
type = "post"
tags = ["hacking-stories", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/hacking-stories/virus-cold"


[extra]
lang = "en"

comment ="""
Newsletter:

v2021.45

Last of the year

A lot of people joined us recently and may have missed
-> top 10 post of the year



hacking story



---------------------------------------------------------------------------------------------------



Sarah est un developeus qui vient de se faire larguer
du coup elle est mal, et sa productivite au travail en prend en coup

Cote sarah:
Parler de l'attaque

- quel type de malware
- quel algorithme de chiffrement
- distribution
- binary's metadata
- activate at a special date
- local privilege elevation



cote bob: parler de la defense:
- aucun audit des dependences
- pas de backups
- pas d'assurance
- systemes non mis a jour


Conclusion:
- and you dear reader, how would you


"""
+++

{{< hacking_stories_disclaimer >}}

Sarah was already in a bad mood this morning: her father died at the beginning of the month. So, when her boss, Mike, convened her in his office, she knew that her troubles were not at an end.

Sarah is a very talented developer working in a small, non-tech company with a little bit less than 30 employees. Among them, 3 are developers, counting Sarah. They principally work on internal tools, supporting the other employees. These 3 developers are managed by Mike that doesn't understand anything about technology but is a close friend of the founder of the company. It's not in Mike's habitude to convene people in his office because he usually is not even here, so Sarah had good reasons to be worried.


As bad news never comes alone, Mike told Sarah that she is no longer part of the company, that she should empty her desk and leave **now**. No reason given, no discussion.

Needless to say that the following months were a descent into hell for Sarah!

No job means no money-in, and so she quickly lost her apartment.

Fast forward a few months later, she has now fully recovered, has a new job, a new life, and new awesome friends in a new city.

But, this event marked her so profoundly that she thought that she needed to get her revenge. While her previous employer was not directly responsible for all that happened to her, they made the situation dramatically worse.

As September was approaching, she thought it was the perfect time to send them a gift for the end of the year: a ransomware to destroy all their data, and thus paralyze the entire company. It would be disastrous for them as they made a good chunk of their revenues during the last quarter.


Her plan was simple. Find a way to deliver a malware to one of the servers, and erase everything on the servers. She knew that the only backups were kept on the servers themselves.


As she read a lot of things on tech news sites about malware embedded in office documents that automagically infect entire companies, she searched on her favorite search engine for more information about that.

Unfortunately, she couldn't find information that is detailed enough to enable her to implement it herself.

She then thought about leaving infected USB keys in the parking lot, like in movies, but quickly discarded the idea as it would end her in jail quickly.


Finally, while reading her favorite tech news aggregator, she read [a very interesting article on supply chain attacks](https://kerkour.com/rust-crate-backdoor/). The idea is to infect a software package used by other projects. Such a package is called a dependency, and the other projects its dependents. Infecting a dependency also infects its dependents. Simple. Powerful.

As she still had a copy of the project on her computer, she inspected the list of dependencies.

"Holy crap" she thought. This is exactly the delivery mechanism I need! Because they were too busy implementing features, her old team never paid attention to their dependencies, but the project relies on thousands of transitive dependencies. If any one of them could be compromised, it would mean a complete takeover over the infrastructure of her former company.

She identified a small dependency that would do the job. She created a fake GitHub profile and convinced the maintainer into giving her the access to edit and publish new versions of the package. Actually, the owner was not interested in maintaining the package anymore. He was too busy with his day job. Thus, he even transferred the package to Sarah's fake profile, and now Sarah only maintainer of the dependency.

It was easier than anticipated.

It was time to start the second phase of her plan: create a ransomware and distribute it through the backdoored dependency.

But there was a twist. She didn't want a ransom. Money was no longer a problem for her. She just wanted them to suffer. Thus, her ransomware was actually not a ransomware, but a one-way encryptor.


Sh coded a quick program that would crawl through the entire filesystem of the infected machine and then use the `AES-GCM-256` cipher to encrypt all the files it encountered, with a random key for each file that is discarded after each encryption operation, turning all the files on the machine on random noise.

It also parses the `.ssh/config` and `.ssh/known_hosts` files of the infected machines in order to find servers to spread to.

Indeed all the servers of her former company were on the same network, all accessible with the same SSH key, and the key was on every server, without a passphrase, for convenience reasons.

Then, the encryptor deposed a `README_RANSOM.txt` file on the `Home` and `Desktop` folders with instructions to send 200 Bitcoins to an address that she created, but destroyed the private key, so nobody will ever be able to access it, forever. Anyway, she knew that the company hadn't the money, nor is insured against cybersecurity risks.

Finally, the malware would delete itself to avoid leaving tracks.

In order to be as effective as possible, she embedded a date in the future into her malware. It would stay hidden on infected machines and activate only on December 25, when everybody will be outside of work. "Merry Christmas" she said with a smile when implementing this routine. This kind of malware is known as a "logic bomb".


To cover her tracks, she made all her possible to make it look like that the malware came from the east, the current evil depicted by mainstream media.

She modified the compilation metadata of the binary to make it match Moscow's timezone, she embedded in the malware a few strings in Cyrilic, and he used Tor plus a VPN server in Kazakhstan to publish her backdoored package.


While she was about the spread her malware, she thought: "Sh*t, what if innocent people get infected and lose their data? What if a hospital gets infected?". To avoid hurting innocent people, she added a slight tweak to her malware that would make it silently crash on all systems other than those of her old company. Thus, if the malware was discovered, analysts would think that it's pure incompetency.


Once everything was ready, she embedded the malware into a `0.0.1` update of the dependency. As soon as her former colleagues would run `npm update`, they would pull the backdoored package.


To this day, she doesn't even know if her virus activated or not. And she doesn't care. Past is past, and she doesn't want to hear anything about her previous life.


<!--
<div style="max-width: 400px" class="center">
  <hr />
</div> -->


<!-- # Mike




"I'm sorry Sarah. The decision come from the decision comes from above"


The following months were very hard. Even if Sarah was not at the top of her form, she was very competent, and the remaining 4 develoeprs were burning out under the amount of work.

He thought many times about leaving the company but he has to take care of his family and a few years remaining to pay for the loan of his house.

One day, he woke up with more than 50 messages and missed call on his phone.


<div style="max-width: 400px" class="center">
  <hr />
</div>


Now it's your turn dear reader, to take a few step back and think about the situation.

How would you have reacted in Sarah's situation?

And how would you have protected yourself from this attack? -->
