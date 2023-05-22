Advanced offensive security with the Rust programming language

hello tout le monde,
Avec les recents cyber security incidents (solarwinds....) j'ai eu l'idee de partager les connaissances que
j'ai acquis.

Le probleme que j'ai remarque: avec mes amis developpeurs c'est qu'isl n'ont simplement
pas idee de ce qui est possible en terme d'attaque et comment certaines fonctionnalites peuvent etre
abusees.

J'ai donc decide de partager mes connaissances acquise ces 7 dernieres annees.

Je pense que Rust est le langage pour tous les gouverner en security: aussi rapide que du C,
Memory safe, et surtout il permet des abstraction de haut niveau pour developer rapidement.




implant: comparer a un drone, et dire qu'il peut se faire jammer (c2 sinhole) et donc ca peut
etre bien de lui donner de l'autonomie

avantage rust: easy to use build tools (as opposed to c, c++ ou python)

Implant: passer en revue les solutions existantes, mais dire pourquoi elles ne nous conviennent pas

implant: canaries (dns, http) for blue team detection

intro: dire qu'on ne va pas aborder 2 points importants d;une intrusion car non necessaires et qu'il y a deja de nombreux
ouvrages de qualite sur le sujet (car ces sujet n'evoluent pas beaucoup)
- social engineering
- physical intrusion
- reverse engineering


implant: reduire la taille de l'executable

implant: "like conventional weapons, most digital weapons have two parts - the missile, or delivery system, responsible for spreading the malicious payload and installing it into machines, and the paylaod itself, which performs the actual attack, such as stealing data or doing other thingsto infected machines.
In this case, the payload was the malicious code that targeteed the Siemens software and PLCs.

intro: dire que rust a un grand succes (pointer vers les polls) aupres des devs, lister les raisons, et dire que pour ces
memes raisons, les gens de la securite devraient l'adorer.
Que certains outils d'attaque ont eux meme des failles

async: montrer le main avec la macro, et la main, et commenter en disant qu'une des core values de Rust
est d'etre explicit plutot qu'implicite, et donc je prefere a la main.

implantL rat, malware, backdoor, implant, beacon, trojan, virus, tout ca est a peu pres la meme chose

intro: 2020 fut une annee plutot interessante, pour tout un tas de raisons, mais en tant que geek, ce que je retiens particulierement c'est les attaques informatiques dont l'impact a ete demultiplie du fait d'un contexte hors-norme.

intro: je veux grace a ce livre transmettre les connaissances et les competences permettant a chaque netizen de comprendre les enjeux de la securite informatique.
Pour cela, on va se mettre dans la peau d'un attaquant.
lister les objectifs d'une attaque informatique (argent, vol d'information, disruption)


code promotion: 42studetnt


threaded network scanner

async, trait object for modules

cryptography
exploit / shellcode
malware

encrypted shellcodes

blockchain attacks
distributed cracking with wasm
xss worm with wasm
github crawler

mobile ? infected android.so

malware, with end-to-end encrypted communication.

supply chain attack -> comment implanter un malware
genre montrer comment veroler un package cargo, ou abuser des compile time options
cargo / npm worm

obfuscated wasm payload
-> https://news.ycombinator.com/item?id=21799737 (The Dark Side of WebAssembly)

fuzzing

encrypting constants with macros?


Plan chapitre:
Pourquoi on veut faire ca (why)
ce que l'on va faire / requirement (what)
background technique (description rust rechnique)
implementation
recapitulatif


montrer ce quil ne faut pas faire, par exemple unbounded concurrency

raconter comme une histoire ? -> genre on veut hacker X compagnie


Reconnaissance:
Dire que la plupart de failles aujourd'hui se trouvent dand la configuration des services cloud


Reconnaissance:
passvie and active reconnaissance


worm: expliquer le code signing et donc pourquoi les virus companions sont moins repandus aujourd'hui


plan d'un chapitre:
- (why) problematique
- (how) background theorique
- (what) Implementation
- Sommaire
- References


introduction: vous pourrez ensuite appliquer ces skills legalement que ce soint dans des red teams,
ou des bug bounties.


intro: il n'a jamais ete aussi facile pour les gens qui aiment la securite offensive
de se former (VMs, CTFs) et de moneyer leurs skills.
Mais d'un autre cote, on voit tres bien que ce n'est pas suffisant.

intro: parler de comment je suis venu a la securite,
et dire qu'aujourd'hui avec Rust, c'est une nouvelle page de l'histoire et de la securite qui s'ecrit.


sommaire: venez rejoindre les rangs de ....

sale page: bien mettre en avant le fait que Rust ouvre une nouvelle page dans l'histoire des langages de programmation

implant: self update

intro: dire que mlagre les avancement de ces dernieres annees (bug bounty...) les hacks nous montrent clairement que l'on est qu'au debut de l'histoire

implant: pivot + persistence

intro + summary + sale apge: montrer que le sujet a un avenir brillant devant lui, faire sentir le lecteur un explorateur


faire un changelog -> sur github


chaque chapitre: un concept rust, un concept securite

phrase d'accroche: la plus intriguante possible

montrer que tout le monde est concerne par la securite

ex: meme vous, cher lecteur, vous pourriez avoir un virus syphonant vos donnees en ce moment meme sans le savoir

Summary:
2 parties.
1 cyber securite:
Stuxnet, solarwind, crypto ransomware, equation group, APT, vous avez surement deja entendu parler de ces attaques, sans avoir jamais su les details....
ces attaques qui frolent avec la magie.
-> trouver une frustration: incomprehension ?
aujourd;hui nous allons nous glisser dans la pea d'un attaquant
2 Rust:

structure:
presenter un enjeu clair pour la fin
des hauts et des bas
a la fin, donner la sensation que l'enjeu a ete repondu


avec ce background j'espere que vous pourrez maintenant savoir comment chacune de vos actions eput etre exploitée, et donc
comment eviter de se faire avoir.


commencer chaque chapitre par un citaiton de sun tzu: https://www.goodreads.com/author/quotes/1771.Sun_Tzu

dans ce livre, nous allons explorer les arcanes de ...


quetes du livre:
- penser comme un attaquant
- comment avoir les skills d'un attaquant

s'inspirer du hacker manifesto: https://en.wikisource.org/wiki/The_Hacker_Manifesto
pour que les gens sentent que je suis un des leurs

dire que la plupart des livres qui parlent de securite informatiques utilisent des outils pre-concus tels que metasploit
et donc on va vite etre limite dans notre champs d'action


ce n'est pas parceque vous ne trouvez pas de faille qu'il n'y en a pas.

un audit peut seulement prouver l'absence de ... (mettre la phrase de disclaimer des audits)


intro: On Cyber Attacks, Cybercrime, and Cyberwar. The main difference is the attacker’s motivation.


EXAMPLE: CYBERWAR / cybercrime / cyberattacks CASES: pour chaque truc, trouver plusieurs exemples,
et raconter une histoire.

"Unlike common cyber threats, which usually simply exploit a present vulnerability, independently from the user or owner, APTs focus on specific high-value targets and their intellectual property or classified information in order to gain strategic benefits.  While common cyber attacks try to gain personal information from a number of victims in a single attack with a few steps, in APTs an attacker carries out multiple steps over a longer time span. APTs are much more sophisticated."


exploitation / intrusion: comparer a une attaque de chateau dire que c'est quand on entre dans le chateau

"An example for direct delivery is to attack the companies’ Internet servers, such as e-mail or DNS. After finding vulnerabilities (such as configuration weaknesses on a publicly reachable server, the attacker can compromise further hosts and install a backdoor. Direct delivery can be reached also by physical access to the target company or by leaving USB sticks on crowded open places in the company in order to lure employees to plug the sticks into their office devices.
Indirect delivery happens through a trusted third party of the target company, causing APT attackers to be hard to trace. A trusted third party can be a supplier or a legitimate website that is frequently visited or contacted by the target company (Chen et al., 2014. In indirect delivery a watering hole (Kim and Vouk, 2015 attack can be used. The goal of these attacks is to compromise a host in the target company by infecting websites the employees frequently visit
The term watering hole attack originates from the tactic of predators in a natural world lurking near watering holes for their victims. According to this concept, the attacker is waiting for the target to visit the infected website. 
"

"Backdoors play a significant role in the next phase. They present a persistent point of entry to the target system for APT attackers.
"

intro: commencer avec une attaque choc pour que le lecteur puisse facilement visualiser

intro: ethical / legal hacking definition et differences de Vulnerability assessment, penetration testing, and Red Teaming

"Red Teams rarely, if ever, run standard vulnerability assessment tools.
These tools are loud and generate more traffic than a Red Team engagement is willing to accept. If a vulnerability assessment tool MUST be used, there should be a question asked as to the type of security assessment being conducted, or they should be run with high focus from a "burned" attack location. Vulnerability assessments are still a critical component to security program but are quite
different in scope and goals of a red team engagement.
"


reutiliser le scanner crée en partie 1 dans l'implant, en le mettant sous forme de lib -> reusable software



enjeux du livre:
- comme on peut le voir sur les attaques informatiques, beaucoup de developpeurs ne pensent pas comme un attaquant
- penser comme un attaquant
- apprendre Rust "avancé"
- apprendre des techniques de securite informatique
enjeux secondaires:
- prouver que Rust est le langage parfait pour la secu


fuzzing:
Quand on est defender cela serta trouver des bug,
quand on est attaquant, cela sert a trouver des bugs... qui pourront etre epxloites.

meme en faisant de l’assembleur a la main il serait difficile de faire mieux
