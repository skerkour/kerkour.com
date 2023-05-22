+++
date = 2022-09-08T06:30:00Z
title = "iPhone = Privacy?"
type = "post"
tags = ["hacking", "privacy"]
authors = ["Sylvain Kerkour"]
url = "/iphone-privacy"

[extra]
lang = "en"
+++

Where were you on Monday, September 8, 2014? How much time do you spend outside of your house every day? How many times have you visited your doctor this year? How many messages have you sent to this other guy when your boyfriend was out of town?

While you may not necessarily have answers to these questions, the data is there, somewhere in the database of some big corporations waiting to be accessed by a disgruntled employee, some hackers, an abusive ex, or government agencies.

But there is hope, Apple, the company who famously had an ad about how they will save the world from a big brother dystopia is here, promoting their devices and services as private and [telling everyone that privacy is one of their core values](https://www.apple.com/privacy/).

But, what about the reality?


## Mobile Network

The first thing to know is that any mobile device can be precisely tracked when connected to the network and that mobile phone carriers not only record this data, [but also sell it](https://www.vice.com/en/article/nepxbz/i-gave-a-bounty-hunter-300-dollars-located-phone-microbilt-zumigo-tmobile).


[In Europe too](https://www.reuters.com/article/uk-health-coronavirus-europe-telecoms/european-mobile-operators-share-data-for-coronavirus-fight-idUKKBN215281), and this is [mandated by law](https://defence-industry-space.ec.europa.eu/eu-space-policy/galileo/caller-location-emergency-communications_en).


With 5G, network hardware vendors trumpet a [positionning accuracy down to the meter, decimeter and centimeter](https://www.ericsson.com/en/blog/2020/12/5g-positioning--what-you-need-to-know)!


It's worth noting that **airplane mode** is just a software thing. The baseband module of your phone (the part that interacts with the mobile network), is a completely distinct system with its own CPU, RAM, Operating System (OS)... When you put your phone in airplane mode, you are simply telling your phone's OS to stop using the mobile network. The baseband system is still on and can be pinged by the mobile network.

So to start, any iPhone (or mobile phone for that matter) is a location tracker that is always on you and tied to your identity (SIM card).


## iOS

Now let's talk about your iPhone's Operating System: iOS.

According to a [study published in March 2021 (PDF)](https://www.scss.tcd.ie/doug.leith/apple_google.pdf) by a team of researchers from Trinity College in Dublin, iOS sends a lot of data about your phone to Apple, such as your phone number, your unique device identifier, your location and your [IMEI number](https://en.wikipedia.org/wiki/International_Mobile_Equipment_Identity).

You can disable <u>some</u> of this data collection (but not all) in `Settings > Privacy > Location Services`.

iOS also uploads a copy of all your pictures, contacts, calendars, notes... to Apple to synchronize it across your devices and keep a backup in case your iPhone is lost or stolen.

Thus, anyone who has access to your iCloud account, whether it be [a hacker](https://www.macrumors.com/2012/08/05/apple-support-allowed-hacker-access-to-reporters-icloud-account/), an Apple employee, or a government agency, has also access to that data.

Again, this can be disabled in `Settings > Apple ID > iCloud`, and then toggle off what you don't want to upload to your iCloud account.

Finally comes Siri, and this is where I think that Apple is the most dishonest. By default, each app that comes preinstalled with the iPhone, and each app you download from the App Store is used to train Siri on your data. The problem is: it's not totally clear what data is uploaded to Apple to train Siri in the cloud, and what data stay on your device for local training.

Anyway, you can go to `Settings > Siri & Search` and disable everything, and then for each app toggle off `Learn from this App`, `Suggestions - Show on Home Screen`, `Suggestions - Suggest App`.

And don't forget to do that each time you install a new app from the App Store.


## Apps

On iOS, [apps are sandboxed](https://support.apple.com/guide/security/security-of-runtime-process-sec15bfe098e/web) which means that they can access only the data that you give them access to (Photos, Contacts...) and can't access the data of other apps or make changes to the device. You can inspect what permissions an app has in `Settings > [App]`, `Settings > Uber` for example.

With iOS 14.5 and later, developers are required to tell you what data they collect about you (**Data Linked to You**) and what data they collect and share with other companies (**Data Used to Track You**).

Here is an example of a weather application with more than [3.4 Millions ratings on the App Store](https://apps.apple.com/us/app/weather-the-weather-channel/id295646461).

![Weather app privacy](https://kerkour.com/2022/iphone-privacy/weather_channel_privacy.png)

It basically says that by using this weather app, they will sell your location data associated with an identifier, certainly your IP address, or your unique device identifier.

Then, someone can buy this data, intersect it with another data set that is linked to the same identifier (IP address), your browsing history collected with web trackers for example, and know exactly what you have in your mind and where you are.

You can limit the amount of data that is tracked as follow: `Settings > Privacy > Tracking` and toggle `Allow apps to Request to Track` to `off`, and `Settings > Privacy > Apple Advertising` and toggle `Personalized Ads` to `off`.

These settings will return a ["zero identifier" to apps requesting the `advertisingIdentifier`](https://developer.apple.com/documentation/adsupport/asidentifiermanager/1614151-advertisingidentifier), which thus cannot be used to identify you as it is no longer unique to your device.


So, don't give apps access to all your data. You don't need to give your location access to this game, and you don't need to give your pictures access to this weather app.



## Innovation

Apple is known to constantly push for new technologies that their competitors then copy.

<!-- One of these innovations is the deprecation of the headphone jack in favor of Bluetooth-only headphones. The problem is that [Bluetooth is used to track your location](https://archive.ph/mNVjQ), and turning off Bluetooth in the control center is not enough, it disables it for 24 hours only.

To completely disable Bluetooth, you need to go to `Settings > Bluetooth` and turn it off from here. -->

One of these innovations announced yesterday for the iPhone 14 event was Satellite communication. Apple is now able to locate an iPhone anywhere in the world (where it would require connectivity to the mobile network, Wifi, or the FindMy network before). While this feature is now reserved for emergencies and the FindMy Network (to locate lost or stolen devices), it's not hard to imagine how it can be abused in the future.

Another innovation, is Apple's plan to scan your pictures to detect eventual [CSAM](https://www.apple.com/child-safety/).

Due to a massive backslash by approximately everybody, they backtracked and announced that they will not enable it for now. But the cat is out of the bag, and it's just a matter of time before they sneakily re-introduce this technology, or [because governments mandate it](https://9to5mac.com/2022/05/11/apples-csam-troubles-may-be-back-as-eu-plans-a-law-requiring-detection/).

There are many problems with this technology. First, it generates [a lot of false positives](https://archive.ph/tL6wk), and thus, a lot of people will get into trouble and have their private pictures reviewed by complete strangers because of the bugs of an automated system. Second, its use can be extended for many other, more worrying, use cases such as political or religious targeting.


## Some Closing Thoughts

With all that information, I think it's reasonable to say that iPhones are far from private. But, as the [alternatives are even worse](https://www.makeuseof.com/samsung-data-breach-should-you-worry/) (Apple's business model is current noy to sell our private data or feed us with ads everywhere), I don't recommend anything else for those who want to minimize their digital trail all while enjoying the convenience of a smartphone, because we still have a certain degree of choice over what data is tracked ("shared").

For those who want more privacy, I recommend an iPad mini with only Wifi (they don't have GPS). Thus, you don't always carry it in your pocket, it doesn't connect to the mobile phone network, and you can reduce your IP address leaks using a VPN.


<!--

You can learn more about the extensive data collected by iOS in this [research paper by Trinity College in Dublin](https://www.scss.tcd.ie/doug.leith/apple_google.pdf). I didn't include their findings here as they study an old version of iOS and things may have changed since.


And we didn't even started talking about data brokers and aggregators that buy data from multiple sources to build detailed profiles and sell them. -->

<!-- But you have to remember that smartphones are the most powerful and widespread tracking device ever invented, and that it's always good to question things that are taken for granted. [Do you **really** need a smartphone?](https://kerkour.com/i-ditched-my-phone). -->
