+++
date = 2023-03-15T11:00:00Z
title = "Apple advanced data protection: A game-changer for privacy?"
type = "post"
tags = ["cryptography", "apple", "security", "hacking", "privacy"]
authors = ["Sylvain Kerkour"]
url = "/apple-advanced-data-protection-privacy"

[extra]
lang = "en"
+++

*Privacy is the foundation of all other freedoms. Without privacy, there can be no freedom of speech, no freedom of association, and no freedom of thought.*

On January 18, 2023, Apple, with iOS 16.3, expanded the global availability of its new feature: [Advanced data protection for iCloud](https://support.apple.com/guide/security/advanced-data-protection-for-icloud-sec973254c5f/web).

Advanced data protection is a pretty big deal when it comes to bringing privacy to the masses by protecting your most important data from unauthorized access, and I have to admit, it even surprised me coming from a tech giant.

By flipping one switch on your Apple devices, you can enable end-to-end encryption (what is end-to-end encryption? More on that later) for almost all of your iCloud data: files, photos, notes, maps, bookmarks, and more.

Even better, iCloud Shared Photo Library, iCloud Drive shared folders, and shared Notes are also end-to-end encrypted when all the participants have advanced data protection enabled.

As we can see [in the documentation](https://support.apple.com/en-us/HT202303#advanced), contacts and calendars, which are pretty sensitive pieces of data, are not end-to-end encrypted with advanced data protection due to some compatibility reasons.

## End-to-end encryption

End-to-end encryption (E2EE) is a method of secure communication and storage that ensures the privacy of data shared between two parties. It's a set of techniques and protocols that application makers and service providers can (and should, in my opinion) implement to prevent unauthorized access to their users' data.

![e2ee](/2021/signatures-modern-end-to-end-encryption/encrypted_communication.svg)


Who want to access users data?

First, there are the hackers. Personal data is valuable for scammers and is used for financial fraud and extortion. A recent example is [the leak of the personal data of 400 million people from Twitter's servers](https://www.bleepingcomputer.com/news/security/hacker-claims-to-be-selling-twitter-data-of-400-million-users/). Furthermore, some disturbed individuals hack cloud accounts of women and celebrities in order to find explicit pictures, such as in [2014 with the hack of the iCloud accounts of hundreds of celebrities](https://www.bbc.com/news/technology-29039294).

But criminals are not the only threat to unencrypted cloud services: in [2019, two Twitter employees were charged with spying for Saudi Arabia](https://www.nytimes.com/2019/11/06/technology/twitter-saudi-arabia-spies.html).

Indeed, whether it be by spying or more traditional methods such as warrants, governments across the world have recently become addicted to easy access to the private data of their citizens and persons of interest. With end-to-end encryption, government agencies have to physically access your devices to get the data, which is closer to the spirit of a free world.

Finally, an advantage of end-to-end encryption less discussed is the protection against automated systems scanning your data. When you use a traditional cloud service such as Google Drive and Google Photos, the data you upload is constantly scanned for "malicious content" (whatever that means, viruses, pirated movies, and music...) and an "AI" may flag your account and permanently block your access to your data. It recently happened to a [father who sent a picture of his son to his doctor for examination](https://www.nytimes.com/2022/08/21/technology/google-surveillance-toddler-photo.html). By taking the picture with his Android phone, it was immediately uploaded to Google Photos' servers and scanned by its "content safety" system, which marked it as child abuse, sent an alert to the police (who investigated and cleared the father, of course), and blocked his account. Even after breaking the news, Google refused to reinstate the account, and the father lost access not only to his files stored in the cloud but also to all his emails and contacts.

Is it normal for a huge company to scrutinize my private pictures and expel me from my entire digital life when all I wanted to do is preserve memories of my young cousin's childhood? Nope, and this kind of stalking is really creepy.

**Want to learn more about how end-to-end encryption works under the hood? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in Rust.**

## But...

Advanced data protection for iCloud is not perfect.

First, if you lose or break all your Apple devices at the same time and lose access to the security key you should have noted and securely stored when setting up advanced data protection, you will lose access to all your data. I consider this a pretty minor risk, as the odds of it happening are quite low, and it can always be neutralized by having an encrypted backup of your data on an external disk.

Second, is [Apple's growing passion for tracking in order to show ads](https://www.bloomberg.com/news/newsletters/2022-08-14/apple-aapl-set-to-expand-advertising-bringing-ads-to-maps-tv-and-books-apps-l6tdqqmg). As the king of consumer hardware, Apple now needs to expand its revenues by selling services to satisfy insatiable financial markets. What are the most profitable companies of today selling? Ads. And so, Apple wants its piece of the cake. The consequences are an increase in the [trackers](https://twitter.com/mysk_co/status/1588308341780262912) present on your [Apple devices](https://sneak.berlin/20220409/apple-is-still-tracking-you-without-consent/) themselves. And as [we saw a few months ago](https://kerkour.com/iphone-privacy), it was far from perfect.

Finally, and even more worryingly, is the technology developed by the same company: [local-content scanning](https://appleprivacyletter.com/). **Apple's devices can scan your private data on your own device** (not on their iCloud servers) for "unsafe content". As we saw previously with Google, these technologies are not reliable and cause more problems than they solve.

While Apple stated that they would not deploy this technology on their devices following the universal backlash after their initial announcement, we all know that companies have short memories and that governments across the world, now knowing that such technology exists, may want to force it into our devices so they can report all our private information, such as with [the EU's chat control](https://mullvad.net/en/chatcontrol/campaign).

## Some closing thoughts

Honestly, I still can't believe how huge this is. Simply by clicking a button, anyone can now end-to-end encrypt their data without any kind of arcane knowledge and protect their data against a long list of threats, which is incredible!

With Advanced Data protection for iCloud and Lockdown mode, Apple is really pushing the status quo on personal digital security. While [I'm usually pretty skeptic](https://kerkour.com/iphone-privacy) when it comes to announces like that. This one is a big deal.

Don't wait, flip the switch and enable advanced data protection on your iCloud account today, for you and all your family (follow [the official guide](https://support.apple.com/en-us/HT212520)).

Just make sure to securely note and store the recovery key of everyone displayed during the setup.


**Want to learn more about how to implement end-to-end encryption in practice? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in  Rust.**
