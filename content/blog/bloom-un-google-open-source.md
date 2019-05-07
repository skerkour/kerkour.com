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
les problèmes non plus. Les enjeux auxquels fait face l'humanité dépassent les Pays, les continents
et menancent notre espèce entière.

C'est donc tous ensemble que nous allons devoir nous unir,
malgré nos différences et notre passé où nos ancêtre se sont parfois opposés.
C'est tous ensemble que nous allons devoir apporter des réponses à la fois aux enjeux de court terme
et ceux du long terme.


Il n'y aura pas de gagnant, pas de vainqueur et donc pas de perdants, la récompense étant simplement
d'éviter que les choses ne tournent affreusement mal pour notre espèce.


### La crise écologiques

La crise écologique étant la plus importante car elle englobe beaucoup de facteurs, que ce soit les
problemes d'accès à de l'eau potable,
le réchauffement climatique, l'extinction d'espèces animales...


A-t-on besoin d'utiliser une voiture de 900kg pour faire 2h de trajet journalier pour finalement
travailler toute la journée derrière un ordinateur ? La question est bien sur rhétorique, la reponse est non.


### La crise sociale


Une question m'obsède: Comment se fait-il que nous devions travailler toujours plus pour avoir accès
à un niveau de vie convenable, quand on voit toutes les richesses produites dans le monde ? Ces
richesses ne cumulent-elles pas ? Si nous sommes organisés en sociétés, alors pourquoi la société dans
son ensemble n'en profite-t-elle pas ? Pourquoi ne travaillons nous simplement pas 2 mois par an ? ou 3 jours par semaine ?

Le mot clé est bien sur le mot *travail*. Les technologies de l'information, la robotique et
l'intelligence artificielle remettent jour après jour en cause notre définition du travail.

Avec l'impôts il est aujourd'hui notre seule méthode de redistribution des richesses.
La technologie fait qu'on a de moins en moins de travail humain pour répondre à nos **besoins**. Et c'est une bonne chose.
À condition de ne pas voir un emploi comme un élément de réussite personnelle, de ne pas voir le chômage comme
une variable qui doit tendre vers 0. Alors que les emplois se déplacent de plus en plus du secteur primaire
vers le secteur tertiaire, le secteur tertiaire ne propose pas d'autant d'occupation que le npmbre d'arrivants.

A-t-on besoin d'un restaurant par personne dans chaque ville ? A-ton besoin de de 100 ingénieurs pour concevoir
un robot pouvant faire le travail de 100 agriculteurs ? NON !


Le chômage est donc voué à auguementer. Or cela va mettre à mal toute l'organisation de notre société.
Par exemple le système des retraites Français, en prenant en compte notre population vieillissante, comment allons nous
faire qunad le nombre *d'inactifs* sera supérieur au nombre *d'actifs* ? (il va falloir trouver une solution
rapidement car c'est pour bientôt, aujourd'hui il y aurait 1,4 *actif* pour 1 *inactif*).

<!-- Les personnes qui mettent en valeur la création d'emplois sont dangereuses. On n'a besoin de "création d'emplois"
que pour maintenir en vie un système malade. -->

<div class="center">
  <img src="/posts/bloom-announcement/wealth_dsitribution.png" height="500"/>

  Distribution des richesses. World economic forum.
</div>


Mais alors que notre


### Le risque pandémique

Je ne vais pas m'étendre sur le sujet par manque de données, de connaissances et de capacité d'action,
mais l'[épidémie du SRAS en 2002](https://fr.wikipedia.org/wiki/Syndrome_respiratoire_aigu_s%C3%A9v%C3%A8re_li%C3%A9_au_coronavirus),
de [grippe A (H1N1) en 2009](https://fr.wikipedia.org/wiki/Grippe_A_(H1N1)_de_2009)
laisse présager un futur turbulent.

Le croisement des technologies de transport, du réchauffement climatique, de l'élevage intensif et de la distribution intensive
d'antibiotiques aussi bien pour notre bétail que pour nous même fait peser un risque réel de maladie infectieuse face à laquelle nous serions impuissants.

**Les antibiotiques, c’est pas automatique !**


## Open source, access, data ...

A l'heure du numérique où la copie et la diffusion d'un document digital ne coûte virtuellement rien,
les volontés de renforcer le droit de la propriété intellectuelle alors que des
aberration effarantes sont deja en place
([taxe pour copie privée](https://fr.wikipedia.org/wiki/R%C3%A9mun%C3%A9ration_pour_copie_priv%C3%A9e),
les derives des ayants droits sur youtubes,
[les derives des journaux scientifiques](https://www.youtube.com/watch?v=WnxqoP-c0ZE),
les [copyright trolls](https://en.wikipedia.org/wiki/Copyright_troll)...).
et dont les externalites negaitves ne sont jamais, ne serait-ce que, mentionnees, sont "au mieux" de la cupidité
si ce n'est criminel. Oui, bloquer des publications scientifiques derrière des abonnement hors de prix tue.

Toute cette bureaucratie n'est bien sur pas la bienvenue alors que l'humanité fait face à des problèmes majeurs.


La niassance de l'open source / data / access... vient de personnes ne cherchant pas à maximiser leur
profit personnel, mais ayant compris que mettre à disposition gratuitement et librement les fruits de leur travail
ne bénéficie pas seulement à eux, mais aussi à la société, à l'humanité entière.


### Open source

[Open source](https://fr.wikipedia.org/wiki/Open_source): *La désignation open source, ou « code source ouvert », s'applique aux logiciels (et s'étend maintenant aux œuvres de l'esprit) dont la licence respecte des critères précisément établis par l'Open Source Initiative, c'est-à-dire les possibilités de libre redistribution, d'accès au code source et de création de travaux dérivés. Mis à la disposition du grand public, ce code source est généralement le résultat d'une collaboration entre programmeurs.*

L'exemple le plus connus d'open source est [Linux](https://fr.wikipedia.org/wiki/Linux), un système d'exploitation
libre et gratuit qui est utilisés par la majorité des [serveurs](https://fr.wikipedia.org/wiki/Serveur_informatique)
et [des téléphones](https://www.android.com) aujourd'hui.

#### Interêts composés

Quamd un développeur travail sur un projet propriétaire, les fruits de son trvail bénéficient uniquement
à son employeur et dans une moindre mesure à ses clients.

Quamd un développeur contribue à un open source, le fruit de son travail lui beneficie à lui, aux
utilisateurs du projet open source, et au reste de monde qui peut l'utiliser gratuitement, le modifier, l'améliorer.

Chaque entreprise, chaque organisation n'a plus à tout réinventer dans son coin, elle est libre de s'appuyer si elle le veut
sur le travail effectué par d'autres pour y ajouter ce qu'elle estime être de la valeur.


#### Organisations distribuées

Les nouvelles technologies de communication (chat, visioconférences...) permettent aujourd'hui à des organisations et entreprises
de s'affranchir de bureaux classiques. Les membres n'ont plus besoin de se réunir tous sur un même lieu de travail pour finalement
rester la journée derrière un écran. Le personnel n'a plus besoin d'être dans la même ville, le même pays voire le même continent.


Bien sur cela pose des défis au niveau des interractions sociales, mais c'est un premier levier pour
réduire notre consommation énergétique à l'échelle planétaire.



#### Collaboration mondiale

Cette intersection des moyens de communication instantanés et de l'ouverture des moyens de production permettent
une collaboration globale. Les utilisateurs peuvent contribuer, participer à la création des produits qu'ils utilisent.


<div class="center">
  <img src="/posts/bloom-announcement/unemployment-vs-biggest10.jpg" height="300"/>

  Une des 10 plus grosses entreprises mondiale VS chômage mondial.
  Données: <a href="https://donnees.banquemondiale.org/indicateur/SL.UEM.TOTL.ZS?end=2018&start=1991&view=chart" target="_blank" rel="noopener">Banque mondiale</a>
</div>



#### Résilience

Que se passerait-il si demain Google perdait tout intêret économique à faire des affaire et bloque son accès à l'Europe ?
Que se passerait-il si demain Twitter faisait faillite et devait stopper ses services ?
Que se passerait-il si demain un gouvernement tyrannique bloquait WhatsApp ?

La crise financière de 2008 nous a douloureusement rappelé que des entreprises que l'on croyait intouchables
peuvent faire faillite. Or depuis [la situation économique modiale](https://fr.wikipedia.org/wiki/Dette_publique) ne s'est pas améliorée.


L'open source permet aux projets de survivre non suelement à la faillite d'une entreprise mais aussi à une coupure d'internet ou encore
à une guerre économique.

Et puisque ces servies entrent dans le bien commun, qu'ils ne sont pas la propriété d'une entrprise particulière,
il y a un intérêt commun à ce qu'un tyran ne censure pas ces services, et donc de l'opposition.


### Open Access



### Open data

L’open data *ou donnée ouverte est une donnée numérique dont l'accès et l'usage sont laissés libres aux usagers.*


La donnée est aujourd'hui comparée au pétrol, de la même manière que ce dernier est le carburant de nos moteurs à
explosion, la donnée est le carburant des [algorithmes](https://fr.wikipedia.org/wiki/Algorithme).

C'est en récoltant beaucoup de données et en leur donnant du sens que nous pourrons avoir des véhicules
autonomes et ainsi réduire drastiquement le nombre de morts sur les routes.


Le problèmes c'est qu'aujourd'hui ces données, produites par tout le monde, sont propriétés de grandes
entreprises (*les géants de la tech*), et sont utilisées entre autre pour [destabiliser nos organisations politiques](https://fr.wikipedia.org/wiki/Cambridge_Analytica).


Disposer des ces données en accès libre permettrait



### Transparence: vertueuse mais surtout nécessaire

Toutes ces mouvances d'ouverture apportent de fait de la transparence. L'information devient accessible.
N'importe qui, sans distinction aucune peut s'en saisir et l'enrichir.

Mais cette transparence vertueuse est surtout nécessaire, car si les vraies informations se propagent
à la vitesse de la lumière, les fausse aussi (*fake news*). Aujourd'hui mises en avant car elles menacent la stabilité
des sociétés modernes il existe 2 grandes directions pour restreindre leurs effets:

* La censure
* La transparence

La censure signifiant une communication à sens unique, d'une minorité ayant le pouvoir vers les masses.

La transparence quant à elle n'est pas suffisante à ellse seule pour mettre fin aux fausses informations, mais
elle permet au moins d'apporter des faits contradictoires et prouvables.


La première étape de la transparence est de rendre les données publiques. La seconde est de les rendre facile
d'accès, facile à chercher, facile à réutiliser.

Sans transparence, il n'y a pas de vie publique. Sans transparence, il n'y a pas de société.
Sans transparence le *lobbying* ne doit être considéré que comme de la corruption.


## Bloom: le master plan top secret

Bloom se veut l'amorce d'un changement de paradigme dans la maniere dont nous consommons mais surtout dans
la manière dont nous produisons.

Un monde de cooperation plutôt que de competition. On ne peut juste pas continuer a tous concourir pour les mêmes
resources finies, faisant fi des destructions et pollutions engendrées.

C'est à nous de choisir aujourd'hui si nous voulons d'une monde d'esclavage permis par les nouvelles technologies,
ou si nous voulons d'un monde où les technologies sont au service du bien de tous. Cet avenir n'est pas encore
écrit.

Bloom une organisation qui grâce à l'Open source/access/data, redistribuera gratuitement et librement
sa produciton à large échelle en s'appuyant sur les nouvelles techonologies pour le faire à moindre
coût, plutôt que des les utiliser pour privatiser et centraliser des richesses.

Bloom reprend les codes et méthodes de *Startuplaland* ainsi que leurs inclinaison vers l'éxecution
plutôt que pour la bureaucratie et les discussions sans fin pour lesquelles nous n'avons plus le temps.

Bloom propose aujourd'hui les services suivants:

* <a href="https://bloom.sh/drive" target="_blank" rel="noopener">Bloom Drive</a>: un espace de stockage en ligne
* <a href="https://bloom.sh/music" target="_blank" rel="noopener">Bloom Music</a>: un lecteur de musique en ligne lisant les musique directement depuis votre *drive*
* <a href="https://bloom.sh/gallery" target="_blank" rel="noopener">Bloom Gallery</a>: une gallerie pour vos photos et vidéos
* <a href="https://bloom.sh/contacts" target="_blank" rel="noopener">Bloom Contacts</a>: un gestionnaire de contacts
* <a href="https://bloom.sh/notes" target="_blank" rel="noopener">Bloom Notes</a>: une application de prise de notes
* <a href="https://bloom.sh/bitflow" target="_blank" rel="noopener">Bitflow</a>: un gestionnaire de téléchargements, qui permet de télécharger des fichiers directement dans votre *drive*
* <a href="https://bloom.sh/platform/phaser" target="_blank" rel="noopener">Phaser</a>: Un scanner de sécurité automatique pour sites webs

Tous offrant un tier gratuit sans contrepartie (pas de ciblage, pas de publicité...)
et dont le code est disponible librement sur [GitHub](https://github.com/bloom42).


### Contribuer / aider

Nous croyons en un monde ou tout le monde peut contribuer, pas seulement des experts en informatique.
C'est pourquoi nous sommes engagés à faciliter cette collaboration mondiale.


#### Developpement

Vous pouvez contribuer librement au code sur [GitHub](https://github.com/bloom42), et avoir plus d'information
sur nos proejts sur notre site [opensource.bloom.sh](https://opensource.bloom.sh).

##### Rust

[Rust](https://www.rust-lang.org/) est le langage de programmation officiel de Bloom, pour une multitude de raisons, les principales
étant:

* Une langage expressif et moderne
* Un langage open-source et embrassant pleinement les valeurs de Bloom
* Un langage permettant la fois des abstractions de haut niveau et des constructions de bas niveaux, sans coût de performance
* Un langage portable qui peut aussi bien être utilisé pour des services web que pour des robots


Il sera utilisé pour **tous** les projets de Bloom sauf obligation contraire
(Typescript + Vue pour les applicaitons web, Kotlin pour les applciation Android, Swift pour les applications IOS...).


#### Design

Pour contribuer au design de Bloom cela se passe sur le repo [github.com/bloom42/design](https://github.com/z0mbie42/design).


#### Financièrement

Si vous n'avez pas le temps ou les compétences pour contribuer, mais que notre projet vous semble prometteur
vous pouvez contribuer financièrement.

Cet argent sera utilisé pour payer les gens contribuant avec leur temps à un monde plus ouvert:

En devenant *patron*:

<a href="https://www.patreon.com/bloom42" target="_blank" rel="noopener">
  <img src="/imgs/become_a_patron_button.png" height="42"/>
</a>


En faisant un don *Paypal*:

<a href="https://paypal.me/z0mbie42" target="_blank" rel="noopener">
  <img src="/imgs/paypal_donate.gif" height="42"/>
</a>


En faisant un don en crypto-monnaie:

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
