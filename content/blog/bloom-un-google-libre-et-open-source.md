+++
date = 2019-05-01T12:42:42
draft = true
title = "Bloom: un Google libre et Open Source"
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

Pour sortir de l'immobilisme face aux enjeux universels et inédits de notre siècle j'ai créé une alternative gratuite,
libre et open source à Google (et sans publicité ni ciblage): Bloom.
C'est par [ici pour la version en ligne](https://bloom.sh), par [ici pour le code](https://github.com/bloom42)
et par [ici pour  contribuer](#contribuer). Ce n'est que le début d'une grande aventure 🚀


## Des enjeux universels et inédits

Voici une nouvelle qui n'en n'est pas une: le début du XXIème siècle est un tournant décisif dans
l'histoire de l'humanité. Si aujourd'hui la culture n'a plus de frontière (évènement sportifs, art ...)
les problèmes non plus. C'est tous ensemble que nous devons apporter des réponses à la fois aux enjeux du court terme (sociaux)
et ceux du long terme (environnementaux).

<!-- Les enjeux auxquels fait face l'humanité dépassent les Pays, les continents
et menancent notre espèce entière. -->

<!-- Nous devons donc nous unir, malgré nos différences et un passé où nos ancêtre se sont parfois opposés. -->

<!--
Il n'y aura pas de perdants, que des gagnants la récompense étant simplement
d'éviter que les choses ne tournent affreusement mal pour notre espèce. Agir ou subir. -->


### La crise écologiques

La crise écologique est la plus importante car elle englobe beaucoup de facteurs, que ce soit les
problèmes d'accès à de l'eau potable,
le réchauffement climatique, l'extinction d'espèces animales, les exodes climatiques, la pollution des sols et des resources aqueuses,
les maladies respiratoires, le 7ème continent de plastique...

Notre surconsommation entraîne la destruction de notre habitat et nous met en danger.

Le constat est allarmant et l'immobilisme encore plus.
<!--
A-t-on besoin d'utiliser une voiture de 900kg pour faire 2h de trajet journalier pour finalement
travailler toute la journée derrière un ordinateur ? La question est bien sûr rhétorique, la réponse est non. -->


### La crise sociale


Une question m'obsède: Comment se fait-il que nous devions travailler toujours plus pour avoir accès
à un niveau de vie convenable, quand on voit toutes les richesses produites dans le monde ?
<!-- Ces richesses ne se cumulent-elles pas ? Si nous sommes organisés en sociétés, alors pourquoi les sociétés dans -->
<!-- leurs ensembles ne profitent-t-elles pas des ces richesses ? -->
<!-- Pourquoi ne travaillons nous simplement pas 2 mois par an ? ou 3 jours par semaine ? -->

<!-- Le mot clé est bien sûr le mot *travail*. Depuis l'invention de la machine à vapeur jusqu'à la robotique et -->
<!-- l'intelligence artificielle, la technologie remet en cause jour après jour notre définition du mot *travail*. -->

Avec l'impôt le travail est aujourd'hui notre seul moyen de redistribution des richesses.
La technologie fait qu'on nécessite de moins en moins de travail humain pour répondre à nos **besoins**.

<!-- Moins de travail signifiant moins de revenu

Le chômage est donc voué à auguementer. Or cela va mettre à mal toute l'organisation de nos sociétés.
Par exemple le système des retraites Français, en prenant en compte une population vieillissante, comment allons nous
faire quand le nombre *d'inactifs* sera supérieur au nombre *d'actifs* ? (il va falloir trouver une solution
rapidement car c'est pour bientôt, aujourd'hui il y aurait 1,4 *actif* pour 1 *inactif*). -->

Deux solutions se présentent:

* Consommer toujours plus
* Mieux redistribuer les richesses

Consommer toujours plus - économies et sociétés basées sur la croissance - n'est évidemment pas compatible avec la crise écologique.

Moins de nécessité de production = moins de travil = + de chômage et donc moins de richesses distribuées.

<div class="center">
  <img src="/posts/bloom-announcement/wealth_dsitribution.png" height="500"/>

  Distribution des richesses. World economic forum.
</div>

La question est donc la suivante: **Comment mieux redistribuer les richesses de manière à améliorer le niveau de vie
universel, sans ccontinuer à accélérer notre autodestruction du fait de la surconsommation ?**


Le printemps arabe, les gilets jaunes, les populations dont la qualité de vie ne bénéficie pas des richesses qu'elles
produisent remettent en cause le <a href="https://fr.wikipedia.org/wiki/Du_contrat_social" target="_blank" rel="noopener">contrat social</a>.
Il est impératif de rapidement trouver un nouveau système, un système où tout le monde y retrouve son compte, pour faire face ensemble à la
crise écologique.


<!-- Et c'est une bonne chose. -->
<!-- À condition de ne pas voir un emploi comme un élément de réussite personnelle, de ne pas voir le chômage comme
une variable qui doit tendre vers 0. Alors que les emplois se déplacent de plus en plus du secteur primaire
vers le secteur tertiaire, le secteur tertiaire n'absorbant pas tous les arrivants.
À cela s'ajoute les impératifs écologiques dont la *seule solution* est la réduction drastique de notre consommation
(énergétique et en resources) et donc laisse présager la réduction d'emplois nécessaires à la production. -->

<!-- A-t-on besoin d'un restaurant par personne dans chaque ville ? -->
<!-- A-ton besoin de de 100 ingénieurs pour concevoir un robot pouvant faire le travail de 100 agriculteurs ? NON ! -->

<!-- <div class="center">
  <img src="/posts/bloom-announcement/unemployment-vs-biggest10.jpg" height="300"/>

  Une des 10 plus grosses entreprises mondiale VS chômage mondial.
  Données: <a href="https://donnees.banquemondiale.org/indicateur/SL.UEM.TOTL.ZS?end=2018&start=1991&view=chart" target="_blank" rel="noopener">Banque mondiale</a>
</div> -->

<!-- Comment occuper toutes ces personnes ? -->


<!-- Les personnes qui mettent en valeur la création d'emplois sont dangereuses. On n'a besoin de "création d'emplois"
que pour maintenir en vie un système malade. -->






<!--
### Le risque pandémique

Je ne vais pas m'étendre sur le sujet par manque de données, de connaissances et de capacité d'action,
mais l'[épidémie du SRAS en 2002](https://fr.wikipedia.org/wiki/Syndrome_respiratoire_aigu_s%C3%A9v%C3%A8re_li%C3%A9_au_coronavirus) et celle de [grippe A (H1N1) en 2009](https://fr.wikipedia.org/wiki/Grippe_A_(H1N1)_de_2009)
laissent présager un futur turbulent.

Le croisement des technologies de transport, du réchauffement climatique, de la distribution intensive
d'antibiotiques aussi bien pour notre bétail que pour nous même fait peser un risque réel de maladie infectieuse face à laquelle nous serions impuissants.

**Les antibiotiques, c’est pas automatique !**
-->


## Open source, access, data ...

A l'heure du numérique où la copie et la diffusion d'un document digital ne coûte virtuellement rien,
les volontés de renforcer le droit de la propriété intellectuelle alors que des
aberration effarantes sont deja en place
([taxe pour copie privée](https://fr.wikipedia.org/wiki/R%C3%A9mun%C3%A9ration_pour_copie_priv%C3%A9e),
les derives des ayants droits sur youtubes,
[les derives des journaux scientifiques](https://www.youtube.com/watch?v=WnxqoP-c0ZE),
les [copyright trolls](https://en.wikipedia.org/wiki/Copyright_troll)...)
et dont les externalités négaitves ne sont jamais, ne serait-ce que mentionnées, sont "au mieux" de la cupidité
si ce n'est criminel. Oui, les brevets sur les médicaments ou les semences tuent.

**Toute cette bureaucratie n'est bien sûr pas la bienvenue alors que l'humanité fait face à des problèmes majeurs.**


La naissance de l'open source / data / access... vient de personnes rejetant les dérives du *Copyright* et ne cherchant pas à maximiser leur
profit personnel, mais ayant compris que mettre à disposition gratuitement et librement des fruits de leur labeur ou passion
ne bénéficie pas seulement à elles, mais aussi à la société, à l'humanité entière.


### Open source

[Open source](https://fr.wikipedia.org/wiki/Open_source): *La désignation open source, ou « code source ouvert », s'applique aux logiciels (et s'étend maintenant aux œuvres de l'esprit) dont la licence respecte des critères précisément établis par l'Open Source Initiative, c'est-à-dire les possibilités de libre redistribution, d'accès au code source et de création de travaux dérivés. Mis à la disposition du grand public, ce code source est généralement le résultat d'une collaboration entre programmeurs.*

L'exemple le plus connu d'open source est [Linux](https://fr.wikipedia.org/wiki/Linux), un système d'exploitation
libre et gratuit qui est utilisés par la majorité des [serveurs](https://fr.wikipedia.org/wiki/Serveur_informatique)
et [des téléphones](https://www.android.com) aujourd'hui.

#### Interêts composés

Quand Bob travaille sur un projet propriétaire, les fruits de son trvail bénéficient
à son employeur et dans une moindre mesure à ses clients. Le jour où il change d'emploi, il perd tout le travail effectué
précédemment.

Quand Jacque contribue ou travaille sur un projet open source, le fruit de son travail lui beneficie à lui, aux
utilisateurs du projet open source et au reste de monde qui peut l'utiliser gratuitement, le modifier, l'améliorer.
Quand Jacque change d'emploi, ou de projet, le résultat de son précédent emploi continue à lui être bénéfique.

Ainsi les richesses produites se cumulent plutôt que de devoir être re-crées par chaque acteur.

<!-- Chaque entreprise, chaque organisation, n'a plus à tout réinventer dans son coin, elle est libre de s'appuyer si elle le veut
sur le travail effectué par d'autres pour y ajouter ce qu'elle estime être de la valeur. -->


#### Organisations distribuées

Les nouvelles technologies de communication (chat, visioconférence...) permettent aujourd'hui à des organisations et entreprises
de s'affranchir des bureaux classiques. Les membres n'ont plus besoin de se réunir tous sur un même lieu de travail pour finalement
rester la journée derrière un écran. Le personnel n'a plus besoin d'être dans la même ville, le même pays voire le même continent.

Bien sûr cela pose des défis au niveau des interractions sociales, mais c'est un premier levier pour
réduire notre consommation énergétique à l'échelle planétaire.


#### Collaboration mondiale

Cette intersection des moyens de communication instantanée et de l'ouverture des moyens de production permettent
une collaboration globale. Les utilisateurs peuvent contribuer, participer à la création des produits qu'ils utilisent.
Ils ne sont plus simplement des consommateurs, mais acteurs.


Entrer dans un modèle de collaboration plutôt que de compétition est la seule solution pour limiter
la surexploitation de notre environnement et donc limiter notre propre empoisonnement.

**La collaboration c'est avant tout un état d'esprit,
celui de créer et de profiter ensemble, que nous soyons employé
ou non.**


#### Résilience

<a href="https://nest.com/whats-happening" target="_blank" rel="noopener">Que se passerait-il si demain Google perdait tout intêret économique à faire des affaire et bloque son accès à l'Europe ?</a>

<a href="https://techcrunch.com/2015/10/21/twitter-ceo-dorsey-apologizes-to-developers-says-he-wants-to-reset-relations" target="_blank" rel="noopener">Que se passerait-il si demain Twitter faisait faillite et stoppait ses services ?</a>

<a  href="https://theintercept.com/2016/05/02/whatsapp-used-by-100-million-brazilians-was-shut-down-nationwide-today-by-a-single-judge" target="_blank" rel="noopener"> Que se passerait-il si demain un gouvernement tyrannique bloquait WhatsApp ?</a>

La crise financière de 2008 nous a douloureusement rappelé que des entreprises que l'on croyait intouchables
peuvent faire faillite. Or depuis [la situation économique modiale](https://fr.wikipedia.org/wiki/Dette_publique) ne s'est pas améliorée.


L'open source permet aux projets de survivre non seulement à la faillite d'une entreprise mais aussi à une coupure d'internet ou encore
à une guerre économique.

Et puisque ces services entrent dans le bien commun, qu'ils ne sont pas la propriété d'une entrprise particulière,
ils deviennent beaucoup plus résilients à des tentatives de censure.


#### Un tremplin pour l'éducation

L'éducation façonne nos sociétés. Lui imposer des limites artificielles (*Copyright*, prix excessifs,
<a  href="https://fr.wikipedia.org/wiki/Gestion_des_droits_num%C3%A9riques" target="_blank" rel="noopener">DRM</a> ...)
n'est que la garantie d'une mort lente et douloureuse.

L'Open source profite triplement à l'éducation:
* En réduisant drastiquement les coûts
* En améliorant la qualité des matériaux éducatifs
* En permettant d'étudier librement les systèmes éxistants


### Open data

L’open data *ou donnée ouverte est une donnée numérique dont l'accès et l'usage sont laissés libres.*


La donnée est aujourd'hui comparée au pétrole, de la même manière que ce dernier est le carburant de nos moteurs à
explosion, la donnée est le carburant des [algorithmes](https://fr.wikipedia.org/wiki/Algorithme).

C'est en récoltant beaucoup de données et en leur donnant du sens que nous pourrons avoir des véhicules
autonomes et ainsi réduire drastiquement le nombre de morts sur les routes. C'est grâce à la donnée que
nous pouvons grandement automatiser la détection de maladies. C'est grâce à la donnée que nous pouvons
optimiser des flux logistique ou énergétique et ainsi réduire notre trop grande consommaiton énergétique.


Le problèmes c'est qu'aujourd'hui ces données, produites par tout le monde, sont propriété de grandes
entreprises (*les géants de la tech*), et sont utilisées entre autre pour [déstabiliser nos organisations politiques](https://fr.wikipedia.org/wiki/Cambridge_Analytica).


Disposer des ces données en accès libre permettrait de multiplier la rapidité d'innovation dans les domaines
de la robotique, de la médecine, des transoprts etc... mais surtout de grandement réduire les besoins en resource dans tous les domaines.

À l'heure où nous déléguons de plus en plus notre mémoire à des entreprises dans des sillots privés nous n'avons
absolument aucune garantie quant à l'avenir de cette mémoire, tant au niveau de sa destruction qu'au niveau
de sa modification.


### Open Access

*L’open access (ou aussi « libre accès », ou encore « accès ouvert ») à la littérature scientifique est un mode de diffusion des articles de recherche sous forme numérique, gratuite et dans le respect du droit d’auteur.*

[Le monde de la publication scientifique est gangréné](https://www.echosciences-grenoble.fr/articles/la-controverse-en-matiere-de-publication-des-articles-scientifiques).

Il n'y a rien à dire, tout le monde voit l'intérêt pour le bien commun de publier publiquement et gratuitement les résultats
des chercheurs financés aussi bien par le secteur public que privé.

Je recommande le livre [Open Acces de Peter Suber](http://ia601805.us.archive.org/17/items/9780262517638OpenAccess/9780262517638_Open_Access.pdf) pour approfondir le sujet.


### Transparence: vertueuse mais surtout nécessaire

Toutes ces mouvances d'ouverture apportent de fait de la transparence. L'information devient accessible.
N'importe qui, sans distinction aucune peut s'en saisir, l'enrichir, lui donner du sens.

Mais cette transparence vertueuse est surtout nécessaire, car si les vraies informations se propagent
à la vitesse de la lumière, les fausse aussi (*fake news*). Aujourd'hui mises en avant car elles menacent la stabilité
de nos sociétés il existe 2 grandes directions pour restreindre leurs effets:

* La censure de masse
* La transparence

D'un côté il est facile de comprendre pourquoi la censure de masse est néfaste, d'un autre côté la transparence n'est pas suffisante à elle seule pour mettre fin aux fausses informations. Elle permet avant tout d'apporter des faits contradictoires et prouvables (contrairement aux *fake news*).


La première étape de la transparence est de rendre les données publiques. La seconde est de les rendre facile
d'accès, facile à chercher, facile à réutiliser.

Sans transparence, il n'y a pas de vie publique. Sans transparence, il n'y a pas de société stable. La transparence
est le fondement même du contre-pouvoir.
<!-- Sans transparence le *lobbying* ne doit être considéré que comme de la corruption. -->


## Bloom: le master plan top secret

Bloom se veut l'amorce d'un changement de paradigme, dans la manière dont nous consommons mais surtout dans
la manière dont nous produisons.

Un monde de collaboration plutôt que de compétition. Nous n'avons simplement pas le luxe de pouvoir continuer à concourir pour les mêmes
resources finies, faisant fi des destructions et pollutions engendrées.

C'est à nous de choisir aujourd'hui si nous voulons travailler toujours plus pour entretenir et éxhacerber des inégalités déjà criantes,
ou si nous voulons d'un monde où les technologies sont au service du bien de tous. Cet avenir n'est pas encore
écrit.

Bloom est une organisation qui grâce à l'Open source/access/data..., redistribue gratuitement et librement
sa production à large échelle en s'appuyant sur les nouvelles technologies pour le faire à moindre
coût, plutôt que des les utiliser comme outil impérialiste.

Bloom suit la *religion de Startuplaland*: l'**l'éxecution radicale
plutôt que la bureaucratie, une expérience utilisateur sans faille et une culture forte pour le produit**. Avec
pour objectif une meilleure qualité de vie commune.

Nous avons choisi la <a href="https://fr.wikipedia.org/wiki/Copyleft" target="_blank" rel="noopener">license virale</a>
<a href="https://www.gnu.org/licenses/agpl-3.0.fr.html" target="_blank" rel="noopener">AGPL v3.0</a> pour la plupart de nos créations pour encourager d'autres personnes
à faire de même. Pas d'*open core*, pas d'entourloupe.


Bloom propose aujourd'hui les services en ligne suivants:

* <a href="https://bloom.sh/drive" target="_blank" rel="noopener">Bloom Drive</a>: un espace de stockage en ligne
* <a href="https://bloom.sh/music" target="_blank" rel="noopener">Bloom Music</a>: un lecteur de musique en ligne lisant les musique directement depuis votre *drive*
* <a href="https://bloom.sh/gallery" target="_blank" rel="noopener">Bloom Gallery</a>: une gallerie pour vos photos et vidéos
* <a href="https://bloom.sh/contacts" target="_blank" rel="noopener">Bloom Contacts</a>: un gestionnaire de contacts
* <a href="https://bloom.sh/notes" target="_blank" rel="noopener">Bloom Notes</a>: une application de prise de notes
* <a href="https://bloom.sh/bitflow" target="_blank" rel="noopener">Bitflow</a>: un gestionnaire de téléchargements, qui permet de télécharger des fichiers directement dans votre *drive*
* <a href="https://bloom.sh/platform/phaser" target="_blank" rel="noopener">Phaser</a>: un scanner de sécurité automatique pour sites webs

Tous offrant un tier gratuit sans contrepartie (pas de ciblage, pas de publicité...)
et dont le code est disponible librement sur [GitHub](https://github.com/bloom42).


**Bloom est le Linux du XXIème siècle. Bloom est le Wikipedia des services.**

Nous n'avons pas encore de réponse concernant la gouvernance mais nous réfléchissons à un système
de gouvernance ouvert, multi-latéral tout en préservant la sécurité et l'intergrité du systèmes
(ce que ne permet pas par exemple la fédération simple, comme le montre les spams dans le monde des emails).


### Contribuer

Nous croyons en un monde ou toute personne peut contribuer, pas seulement des experts.
**Où les consommateurs et citoyens deviennent acteurs.**
C'est pourquoi nous sommes engagés à faciliter cette collaboration mondiale et sans barrières.



#### Communication

Un chat Discord est disponible ici: <a href="https://discord.gg/HmDQDGv" target="_blank" rel="noopener">https://discord.gg/HmDQDGv</a>.


Pour rester informé des dernières mises à jours suivez nous sur Twitter <a href="https://twitter.com/bloom_cloud" target="_blank" rel="noopener">@bloom_cloud</a>.


#### Développement

Pour contribuer au code cela se passe sur [github.com/bloom42](https://github.com/bloom42).

Plus d'informations sont disponibles sur notre site [opensource.bloom.sh](https://opensource.bloom.sh).

##### Rust

[Rust](https://www.rust-lang.org/) est le langage de programmation officiel de Bloom, pour une multitude de raisons, les principales
étant:

* Une langage expressif et moderne
* Un langage open-source et embrassant pleinement les valeurs de Bloom
* Un langage permettant la fois des abstractions de haut niveau et des constructions de bas niveaux, sans coût de performance
* Un langage portable qui peut aussi bien être utilisé pour des services web que pour des robots
* Tous nos services doivent pouvoir être auto-hébergés sur un <a href="https://www.raspberrypi.org" target="_blank" rel="noopener">Raspberry pi</a>


Il sera utilisé pour **tous** les projets de Bloom sauf impératif contraire
(Typescript + Vue pour les applications web, Kotlin pour les applications Android, Swift pour les applications IOS...).


#### Design

Pour contribuer au design de Bloom cela se passe sur le repo [github.com/bloom42/design](https://github.com/z0mbie42/design).


#### En partageant

Diffusez les idées, débattez avec vos amis, paratgez Bloom sur les réseaux sociaux !

Nous ne sommes encore qu'au début de l'aventure et nous faire connaître nous aide ééénooormément !

#### Financièrement

Si vous n'avez pas le temps ou les compétences pour contribuer, mais que notre projet vous semble prometteur
vous pouvez contribuer financièrement.

Cet argent sera utilisé pour financer les personnes contribuant avec leur temps à un monde plus ouvert:

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


ou tout simplement en souscrivant à un abonnement sur <a href="https://bloom.sh" target="_blank" rel="noopener">bloom.sh</a>.


Un grand merci aux personnes ayant déjà contribué:
*


### En résumé

En résumé le master plan est le suivant:

1. Créer des logiciels libres et vendre l'hébergement ainsi la sécurité des données hébergées<br />
2. Avec cet argent réduire le prix de ces logiciels, libérer la donnée et l'accès a la connaissance scientifique<br />
3. Avec cet argent et cette communauté créer l'infrastructure publique pour faire fonctionner ces logiciels et héberger ces données publiques

Notre mission ? Faire du monde une meilleure place grâce à des technologies ouvertes, par le peuple, pour le peuple.

Mais chuuut, c'est un secret 🤫

-------

*Pour triompher, le mal n’a besoin que de l’inaction des gens de bien...*

♪ [MC Solaar - Sauvez le monde](https://www.youtube.com/watch?v=dERx6uFUf6o)

📖 [21 leçons pour le XXIe siècle, *Yuval Noah Harari*](https://www.ynharari.com/fr/book/21-lessons/)

🎬 [Internet ou la révolution du partage | ARTE](https://www.youtube.com/watch?v=gr2uo-Nwot0)
