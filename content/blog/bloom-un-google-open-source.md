+++
date = 2019-03-01T12:42:42+02:00
draft = true
title = "Bloom: un Google Open Source"
tags = ["rust", "rustlang", "opensource", "google", "free software"]

[extra]
lang = "fr"
+++

*Il ne faut pas compter sur ceux qui ont crée les problèmes pour les résoudre.*

🇬🇧 [English version here](/blog/bloom-an-open-source-google)

1. [Des enjeux universels et inédits](#des-enjeux-universels-et-inedits)
2. [Open source, access, data ...](#open-source-access-data)
3. [Bloom: le master plan top secret](#bloom-le-master-plan-top-secret)

-------

## TL;DR

Pour amorcer une réponse aux enjeux inédits de notre siècle j'ai créé une alternative gratuite,
libre et open source à Google (et sans publicités / ciblage): Bloom.
C'est par [ici pour la version en ligne](https://bloom.sh), par [ici pour le code](https://github.com/bloom42)
et par [ici pour aider / contribuer](#contribuer-aider). Ce n'est que le début d'une grande aventure 🚀


## Des enjeux universels et inédits

Voici une nouvelle qui n'en n'est pas une: le début du XXIème siècle est un tournant décisif dans
l'histoire de l'humanité. Si aujourd'hui la culture n'a plus de frontière (evenement sportifs, art, )
les problemes non plus. Les enjeux auxquels fait face l'humanité aujourd'hui dépassent les Pays, les continents
et menancent l'humanité entière.

C'est donc tous ensemble que nous allons devoir nous unir,
malgré nos différences et notre passé où nos ancêtre se sont parfois opposés.
C'est tous ensemble que nous allons devoir réponse a la fois aux enjeux de court terme et ceux du long terme.


Il n'y aura pas de gagnant, pas de vainqueur et donc pas de perdants, la récompense étant simplement
notre survie.


### La crise écologiques

La crise écologique étant la plus importante car elle englobe beaucoup de facteurs, que ce soit les
problemes d'accès à de l'eau potable,
le réchauffement climatique, l'extinction d'espèces animales...


A-t-on besoin d'utiliser une voiture de 900kg pour faire 2h de trajet journalier pour finalement
travailler toute la journée derrière un ordinateur ? La question est bien sur rhétorique, la reponse est non.

### La crise sociale

- probleme de redistribution des richesses (crise sociale) qui est accelere par l'automatisation et
les effets d'echelles des nouvelles technologies

Les technologies de l'information, la robotique et l'intelligence artificielle remettent jour après jour
en cause notre rapport avec le travail.


### Le risque pandémique
a la fois lie aux nouvelles technologies de transport et au changement climatique.

Je ne vais pas m'étendre sur le sujet par manque de données, de connaissances et de capacité d'action,
mais l'[épidémie du SRAS en 2002](https://fr.wikipedia.org/wiki/Syndrome_respiratoire_aigu_s%C3%A9v%C3%A8re_li%C3%A9_au_coronavirus),
de [grippe A (H1N1) en 2009](https://fr.wikipedia.org/wiki/Grippe_A_(H1N1)_de_2009)
laisse présager un futur turbulent.

Le croisement des technologies de transport, du réchauffement climatique, de l'élevage intensif et de la distribution intensive
d'antibiotiques fait peser un réel risque de maladie infectieuse face à laquelle nous serions impuissants.


## Open source, access, data ...

A l'heure du numérique où la copie et la diffusion d'un document digital ne coûte virtuellement rien,
les volontés de renforcer le droit de la propriété intellectuelle alors que des abherations effarantes sont deja en place
([taxe pour copie privée](https://fr.wikipedia.org/wiki/R%C3%A9mun%C3%A9ration_pour_copie_priv%C3%A9e),
les derives des ayants droits et des strikes youtubes,
[les derives des journaux scientifiques](https://www.youtube.com/watch?v=WnxqoP-c0ZE),
les [copyright trolls](https://en.wikipedia.org/wiki/Copyright_troll)...).
et dont les externalites negaitves ne sont jamais, ne serait-ce que, mentionnees, sont "au mieux" de la cupidité
si ce n'est criminel. Oui, bloquer des publications scientifiques derrière des abonnement hors de prix tue.

Toute cette bureaucratie n'est bien sur pas la bienvenue alors que l'humanite fait face à des problèmes majeurs.


La niassance de l'open source / data / access... vient de personnes ne cherchant pas a maximiser leurs
profits personnels, mais ont compris que mettre à disposition gratuitement et librement les fruits de leur travail
ne bénéficiaient pas seulement à eux, mais a la société, l'humanité entière.

### Open source

[Open source](https://fr.wikipedia.org/wiki/Open_source): La désignation open source, ou « code source ouvert », s'applique aux logiciels (et s'étend maintenant aux œuvres de l'esprit) dont la licence respecte des critères précisément établis par l'Open Source Initiative, c'est-à-dire les possibilités de libre redistribution, d'accès au code source et de création de travaux dérivés. Mis à la disposition du grand public, ce code source est généralement le résultat d'une collaboration entre programmeurs.

L'exemple le plus connus d'open source est [Linux](https://fr.wikipedia.org/wiki/Linux), un système d'exploitation
libre et gratuit qui est utilisés par la majorité des [serveurs](https://fr.wikipedia.org/wiki/Serveur_informatique)
et [des téléphones](https://www.android.com) aujourd'hui.


<div class="center">
  <img src="/posts/bloom-announcement/unemployment-vs-biggest10.jpg" height="300"/>

  Une des 10 plus grosses entreprises mondiale VS chômage mondial.
  Données: <a href="https://donnees.banquemondiale.org/indicateur/SL.UEM.TOTL.ZS?end=2018&start=1991&view=chart" target="_blank" rel="noopener">Banque mondiale</a>
</div>



#### Organisations distribuées


### Open Access



### Open data


### Open governance

rapelle des definitions


parler de la collaboration mondiale..



### Transparence: vertueuse mais surtout nécessaire

Toutes ces mouvances d'ouverture mettent apportent de fait de la transparence. L'information devient accessible.
N'importe qui, sans distinction aucune peut s'en saisir et l'enrichir.

Mais cette transparence vertueuse est surtout nécessaire, car si les vraies informations se propagent
à la vitesse de la lumière, les fausse aussi (*fake news*). Aujourd'hui mises en avant car elles menacent la stabilité
des sociétés modernes il existe 2 grandes directions pour restreindre leurs effets:

* La censure
* La transparence

La censure signifiant le retour à l'époque de la télévision où la communication se faisait à sens unique,
d'une minorité vers les masses.

La transparence quant à elle n'est pas suffisante seule pour mettre fin aux fausses informations, mais
elle permet au moins d'apporter des faits contradictoires et prouvables.


La première étape de la transparence est de rendre les données publiques. La seconde est de les rendre facile
d'accès, facile à chercher, facile à réutiliser.

Sans transparence le *lobbying* ne doit être considéré que comme de la corruption. Sans transparence,
il n'y a pas de vie publique. Sans transparence, il n'y a pas de société.



## Bloom: le master plan top secret

Bloom se veut l'amorce d'un changement de paradigme dans la maniere dont nous consommons mais surtout dans
la manière dont nous produisons.

Vous l'avez compris, tout ce que créera Bloom sera Open source/access/data...

Bloom est aujourd'hui composé des services suivants:

* <a href="https://bloom.sh/drive" target="_blank" rel="noopener">Bloom Drive</a>: un espace de stockage en ligne
* <a href="https://bloom.sh/music" target="_blank" rel="noopener">Bloom Music</a>: un lecteur de musique en ligne lisant les musique directement depuis votre *drive*
* <a href="https://bloom.sh/gallery" target="_blank" rel="noopener">Bloom Gallery</a>: une gallerie pour vos photos et vidéos
* <a href="https://bloom.sh/contacts" target="_blank" rel="noopener">Bloom Contacts</a>: un gestionnaire de contacts
* <a href="https://bloom.sh/notes" target="_blank" rel="noopener">Bloom Notes</a>: une application de prise de notes
* <a href="https://bloom.sh/platform/phaser" target="_blank" rel="noopener">Phaser</a>: Un scanner de sécurité automatique pour sites webs.


### Contribuer / aider

Je crois en un monde ou tout le monde peut contribuer. La mission de Bloom est d'améliorer

 Bloom believes in a world where everyone can contribute. Our mission is to change all creative work from read-only to read-write. When everyone can contribute, users become contributors and we greatly increase the rate of human progress.

We want to make it as easy as possible for Bloom users to become Bloom contributors

#### Developpement


#### Design

#### Financièrement

Si vous n'avez pas le temps ou les compétences pour contribuer, vous pouvez toujours faire des donnations:

En devenant un *patron*:

<a href="https://www.patreon.com/bloom42" target="_blank" rel="noopener">
  <img src="/imgs/become_a_patron_button.png" height="42"/>
</a>


En faisant un don *Paypal*:

<a href="https://paypal.me/z0mbie42" target="_blank" rel="noopener">
  <img src="/imgs/paypal_donate.gif" height="42"/>
</a>


En utilisant les crypto-monnaies:

**BTC**: `38QY24nHRkMxUFsEDobwJU2b5QzuSL39Yb`

**ETH**: `0x5121FE2A1014C4d57FCD2E8C4134A179851aFe6F`

**XMR**: `4GdoN7NCTi8a5gZug7PrwZNKjvHFmKeV11L6pNJPgj5QNEHsN6eeX3DaAQFwZ1ufD4LYCZKArktt113W7QjWvQ7CW7fRk3auob6QWFSgYJ`



### En résumé

En résumé le master plan est:

1. Créer des logiciels payants<br />
2. Avec cet argent créer des logiciels gratuits et libres, libérer la donnée et l'accès a la connaissance scientifique pour le bien commun<br />
3. Avec cet argent et cette communauté créer l'infrastructure publique pour faire fonctionner ces logiciels et héberger ces données publiques


Mais chuuut, c'est un secret 🤫

-------

*Pour triompher, le mal n’a besoin que de l’inaction des gens de bien...*

♪ [MC Solaar - Sauvez le monde](https://www.youtube.com/watch?v=dERx6uFUf6o)

📖 [21 leçons pour le XXIe siècle, *Yuval Noah Harari*](https://www.ynharari.com/fr/book/21-lessons/)
