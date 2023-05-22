+++
date = 2021-11-10T10:00:00Z
title = "Hacking Stories #1 - The Evil Twin"
type = "post"
tags = ["hacking-stories", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/hacking-stories/evil-twin"

[extra]
lang = "en"

comment ="""

"""
+++


{{< hacking_stories_disclaimer >}}


<!--
situation initial: description du hero
et de l'objectif, de la cause
 -->

James is a Computer Science student like many others: brilliant, but a little bit shy, especially with girls. The fact that there are only 3 girls for 25 boys in his class doesn't help.

Among these 3 girls, there is Mary. Black hair. Deep and dark eyes. The kind of girl that makes James' heart rate go crazy when she looks in his direction. Since the first day, James has been secretly in love of Mary. Can we actually call that love as it's a one-way story? I don't know. Anyway, his shyness always prevented him from talking to her.

One day he had an idea: "I should hack the University database to get Mary's phone number and send her an invitation for diner.".

James always had this leaning toward hacking. Since very young, he broke every gadget he had in order to understand how they work and to modify them.

<!--
Le probleme, element perturbateur
-->

He quickly figured out that it may be one of the worst ways to approach a girl, but he persisted:
"What if instead of only hacking Mary's number, I hacked the numbers of all the students of the University and sold it to other students who had a crush?"

The idea seemed very good, and even if it would not make him the next Bill Gate, it would be very entertaining.

<!--
hesitation, parcours chaotique pour arriver a la bonne solution
-->

His first plan was to stay in the University during the night, infiltrate the administration's offices and steal the disk of one of the computers which certainly contained cookies or credentials to access the students' records.

It took him approximately 2 seconds to give up on this idea as there was approximately a probability of 99.99% that it would end him in jail.



<!--
Trouver la bonne solution, et l'etaler
-->


A morning, when arriving at the University, the campus' wifi had some problems: it displayed an error message, and nobody could access the internet.

When you connect to the Univesity's wifi, you have to enter your student's credentials in a captive portal to be able to access the internet. While most students in the world would use their 4G phone as a hotspot, it was a necessity in James' University as the mobile phone coverage was terrible.

An idea started to sprout in James' mind. "What if I could hack the wifi access points of the University and sniff the credentials". After all, everybody on the campus had to enter their credentials to connect to the wifi, every day. Because the wifi routers were publicly visible, James quickly identified the model and started to search for exploits. Unfortunately, he didn't find anything.


After a few days, another idea came: "What if I created a false access point with a malicious captive portal so that when people want to access the internet, they will connect to my captive portal, and I will be able to phish their credentials. Then I would just have to show an error message, and everybody will think that the wifi has problems, like many other times before."


After some Google-fu, it turned out that this kind of attack already exists and is known as an Evil Twin: a malicious access point that record credentials or sniff network traffic.


By luck, James already had all the equipment required to build an Evil Twin access point:
- a Raspberry Pi
- a Battery
- a powerful Alfa Network wifi card (James lives in the countryside where the wifi has problems going through big walls and traversing his gigantic backyard with many trees).


When the weekend started, James frenetically started to put his plan into action and prepare the software required to create the malicious access point and captive portal.

First, a DHCP server. When the computers and smartphones will connect to the wifi hotspot, they will for a local IP address assigned.


Second, a DNS server is needed so that all web requests made by the victims will be redirected to his malicious captive portal. He could have used `bind`, the standard DNS server, but he decided to code his own in Rust with the help of the `trust-dns-server` package. For fun and because understanding how to configure `bind` would have taken him more time.

Finally, a web server that is going to serve the phishing page and save the stolen credentials. For that, he decided to play with the new web framework by tokio's team: `axum`. The web server was simple to implement: James visited the real captive portal of the University, clicked on "View Page Source" and copied the HTML code. Then he just had to serve this HTML to the victims and create the HTTP route that would handle the form submission, save the credentials in an SQLite database, and redirect the victims to the error page.


On Monday morning, James' hands were shaking, and his heart was close to the explosion. He connected his wifi card to the Raspberry Pi and the Raspberry Pi to the battery. On boot, the Raspberry Pi automatically launched the three services (DHCP server, DNS server, and web server) thanks to `systemd`. For the first time of the year, James did not sit at the back of the amphitheater but in the middle.

Indeed, computers and smartphones are dumb. When multiple access points have the same name (SSID) they connect to the one with the strongest signal. As James' wifi card by Alfa Network was way more powerful than the University's access points, all the devices of the amphitheater connected to his evil hotspot when trying to auto-connect to the University's wifi network.


When James saw the error page he crafted a day ago on the screen of the person in front of him, he couldn't refrain from smiling.


At noon, he decided that he had enough credentials and that it was time to execute the last phase of his plan: gathering the private data of the victims he just phished.


For that, he connected to the University's students portal with one of the phished accounts and acceded the personal information page. On this page, the connected user can access all their personal data: full name, birth date, address, phone number, email...


One more time, he clicked "inspect element". This time, instead of copying the HTML he noted all the IDs of the HTML elements wrapping each piece of personal data. With that information, James coded a scraper to automatically download the data of all the persons he just phished.

The scrapper first logged in to the University's portal using a phished pair of credentials in order to fetch an authentication cookie. Then with this cookie, it fetches the personal information page, parses the HTML, and extracts all the personal data in a structured way thanks to the IDs of the HTML elements.

It took James less than 15 minutes to code his scraper in Rust with the `reqwest` and `select` crates. As the scraper was about to perform suspicious activity which could raise suspicion, James decided to cover his tracks: he used the `macchanger` software to change the hardware address of his laptop's wifi card and connected to the wifi with the credentials of one of his victims.

2 minutes later, the scraper had downloaded all the data. James closed his laptop and took the direction of the University's library to continue his day like any other student.


<!-- Succes et conclusion -->

But James is not a criminal. At the end of the day, when he came back home, he started to panic a little bit and imagined the police kicking the front door of his parents. He decided that deleting the stolen data was the safest move and just kept the code.

Just in case ;)
