+++
date = 2019-06-05T01:42:42+02:00
title = "Bloom : un Google libre et open source"
tags = ["bloom", "rust", "open-source", "google", "free-software", "french"]
type = "post"
authors = ["Sylvain Kerkour"]
url = "/bloom-un-google-libre-et-open-source"

[extra]
lang = "fr"
+++

🇬🇧 [English version here](/bloom-a-free-and-open-source-google)

*Il ne faut pas compter sur ceux qui ont créé les problèmes pour les résoudre.*


1. [Des défis universels et inédits (Pourquoi ?)](#des-defis-universels-et-inedits-pourquoi)
2. [Open source, access, data... (Comment ?)](#open-source-access-data-comment)
3. [Bloom : le master plan top secret (Quoi ?)](#bloom-le-master-plan-top-secret-quoi)
4. [En résumé](#en-resume)

-------

## TL;DR

Pour faire face aux défis universels et inédits de notre siècle, nous avons créé un "Google"
libre et open source (et sans publicité ni ciblage) : Bloom - aujourd'hui une suite d'applications de productivité,
il y aura demain une application pour chacuns de vos besoins quotidiens. Nous pensons que l'open source
est la seule voie pour construire un monde durable.

C'est par <a href="https://bloom.sh" target="_blank" rel="noopener">ici pour la version en ligne</a>, par <a href="https://github.com/skerkour/bloom" target="_blank" rel="noopener">ici pour le code</a>
et par [ici pour contribuer](#contribuer). Ce n'est que le début d'une grande aventure 🚀


## Des défis universels et inédits (Pourquoi ?)

Voici une nouvelle qui n'en n'est pas une : le début du XXIe siècle est un tournant décisif dans
l'histoire de l'humanité. Si aujourd'hui la culture n'a plus de frontière (évènements sportifs, art...),
les problèmes non plus. C'est tous ensemble que nous devons apporter des réponses à la fois aux défis du court terme (sociaux)
et ceux du long terme (environnementaux).

<!-- Les défis auxquels fait face l'humanité dépassent les Pays, les continents
et menancent notre espèce entière. -->

<!-- Nous devons donc nous unir, malgré nos différences et un passé où nos ancêtre se sont parfois opposés. -->

<!--
Il n'y aura pas de perdants, que des gagnants la récompense étant simplement
d'éviter que les choses ne tournent affreusement mal pour notre espèce. Agir ou subir. -->


### La crise écologique

La crise écologique est la plus importante car elle englobe beaucoup de problèmes sous-jacents, que ce soit l'accès à l'eau potable,
le réchauffement planétaire, l'extinction d'espèces animales, les exodes climatiques, la pollution des sols et des réserves aqueuses,
les maladies respiratoires, le 7e continent de plastique...

Notre surconsommation entraîne la destruction de notre habitat et nous met en danger. Le constat est alarmant, l'immobilisme encore plus.
<!--
A-t-on besoin d'utiliser une voiture de 900kg pour faire 2h de trajet journalier pour finalement
travailler toute la journée derrière un ordinateur ? La question est bien sûr rhétorique, la réponse est non. -->


### La crise sociale


Une question nous obsède : Comment se fait-il que nous devons travailler toujours plus pour accéder
à un niveau de vie convenable, quand on voit toutes les richesses produites dans le monde ?
<!-- Comment se fait-il -->
<!-- que ce ne soit pas l'objectif prioritaire de chaque pouvoir public ? -->
<!-- Ces richesses ne se cumulent-elles pas ? Si nous sommes organisés en sociétés, alors pourquoi les sociétés dans -->
<!-- leurs ensembles ne profitent-t-elles pas des ces richesses ? -->
<!-- Pourquoi ne travaillons nous simplement pas 2 mois par an ? ou 3 jours par semaine ? -->

<!-- Le mot clé est bien sûr le mot *travail*. Depuis l'invention de la machine à vapeur jusqu'à la robotique et -->
<!-- l'intelligence artificielle, la technologie remet en cause jour après jour notre définition du mot *travail*. -->

Avec l'impôt, le travail est aujourd'hui notre seul moyen de redistribution des richesses.
La technologie fait qu'on nécessite de moins en moins de travail humain pour satisfaire nos **besoins**.

<!-- Moins de travail signifiant moins de revenu

Le chômage est donc voué à auguementer. Or cela va mettre à mal toute l'organisation de nos sociétés.
Par exemple le système des retraites Français, en prenant en compte une population vieillissante, comment allons nous
faire quand le nombre *d'inactifs* sera supérieur au nombre *d'actifs* ? (il va falloir trouver une solution
rapidement car c'est pour bientôt, aujourd'hui il y aurait 1,4 *actif* pour 1 *inactif*). -->

Deux futurs se présentent alors :

* Consommer toujours plus
* Meilleure répartition des richesses

Consommer toujours plus - sociétés basées sur la dette et la croissance - n'est évidemment pas compatible avec la crise écologique.

À l'inverse, la réduction de la nécessité de production = moins de travail = encore plus de chômage et donc moins de richesses redistribuées.

<div class="center">
  <img src="/2019/bloom-a-free-and-open-source-google/wealth_dsitribution.png" height="500"/>

  Distribution des richesses. World economic forum.
</div>

Si la question des dernières décennie était: *Comment produire toujours plus ?*, LA question aujourd'hui est : **Comment améliorer le niveau de vie
universel sans continuer à accélérer notre autodestruction ?**

Continuer à mutualiser les coûts et privatiser les bénéfices est fondamentalement incompatible avec le <a href="https://fr.wikipedia.org/wiki/Du_contrat_social" target="_blank" rel="noopener">contrat social</a> (printemps arabe, les indignés, Brexit, gilets jaunes...).


<!-- Le printemps arabe, le Brexit, les gilets jaunes, les populations dont la qualité de vie ne bénéficie pas des richesses qu'elles
produisent remettent en cause le .
Il est impératif de rapidement trouver un nouveau <!--  système, un système où tout le monde trouve son compte, pour faire face ensemble à la
crise écologique qui nous affecte tous. -->

<!-- **Continuer à mutualiser les coûts et privatiser les bénéfices est fondamentalement incompatible avec notre organisation en sociétés.** -->

<!-- Étonnamment, peu de personnes semblent décidées à trouver des solutions concrètes à ces problèmes. -->

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


## Open source, access, data... (Comment ?)
<!--
A l'heure du numérique où la copie et la diffusion d'un document digital ne coûte virtuellement rien,
les volontés de renforcer le droit de la propriété intellectuelle, alors que des
aberration effarantes sont déjà en place
([taxe pour copie privée](https://fr.wikipedia.org/wiki/R%C3%A9mun%C3%A9ration_pour_copie_priv%C3%A9e),
les derives des ayants droits sur les plateformes de partage de contenu,
[les derives des journaux scientifiques](https://www.youtube.com/watch?v=WnxqoP-c0ZE),
les [copyright trolls](https://en.wikipedia.org/wiki/Copyright_troll)...)
les volontés de renforcer le droit de la propriété intellectuelle dont les externalités
négaitves ne sont jamais, ne serait-ce que mentionnées, sont "au mieux" de la cupidité
si ce n'est criminel. Oui, les brevets sur les médicaments ou les semences tuent. -->

La solution ? Un enfant de 4 ans la trouverait sans problème : **le partage**.

À l'heure du numérique où la copie et la diffusion d'un document digital ne coûte virtuellement rien
et que des
aberrations sont déjà en place
(<a href="https://fr.wikipedia.org/wiki/R%C3%A9mun%C3%A9ration_pour_copie_priv%C3%A9e" target="_blank" rel="noopener">taxe pour copie privée</a>,
les dérives des ayants droits sur les plateformes de partage de contenu,
<a href="https://www.youtube.com/watch?v=WnxqoP-c0ZE" target="_blank" rel="noopener">les dérives des journaux scientifiques</a>,
les <a href="https://en.wikipedia.org/wiki/Copyright_troll" target="_blank" rel="noopener">copyright trolls</a>...), la
<a href="https://saveyourinternet.eu/" target="_blank" rel="noopener">volonté de renforcer le droit de la propriété intellectuelle</a> dont les externalités négatives ne sont jamais mentionnées, est cupide ET criminel. Car oui, les brevets sur les médicaments ou les semences tuent.

**Toute cette bureaucratie n'est bien sûr pas la bienvenue alors que l'humanité fait face à des crises majeures.**


<!-- La naissance de l'open source / data / access... vient de personnes qui rejettent les dérives du *Copyright* et qui ne cherchent pas à maximiser leur
profit personnel, mais qui ont compris que mettre à disposition gratuitement et librement les fruits de leur labeur (ou passion)
ne leur bénéficie pas seulement à elles, mais aussi à la société, à l'humanité entière. -->


### Open source

<a href="https://fr.wikipedia.org/wiki/Open_source" target="_blank" rel="noopener">Open source</a> : *La désignation open source, ou « code source ouvert », s'applique aux logiciels (et s'étend maintenant aux œuvres de l'esprit) dont la licence respecte des critères précisément établis par l'Open Source Initiative, c'est-à-dire les possibilités de libre redistribution, d'accès au code source et de création de travaux dérivés. Mis à la disposition du grand public, ce code source est généralement le résultat d'une collaboration entre programmeurs.*

L'exemple le plus connu d'open source est <a href="https://fr.wikipedia.org/wiki/Linux" target="_blank" rel="noopener">GNU/Linux</a>, un système d'exploitation
libre et gratuit qui est utilisé par la majorité des <a href="https://fr.wikipedia.org/wiki/Serveur_informatique" target="_blank" rel="noopener">serveurs</a>
et <a href="https://www.android.com" target="_blank" rel="noopener">des téléphones</a> aujourd'hui.

<!-- Que votre téléphone repose sur Android ou IOS, il contient forcément
des logiciels open source en plus ou moins grande quantité. Il en va de même pour votre *box* internet, ou encore
n'importe quel site web que vous pouvez consulter. -->


#### Intérêts composés

Quand Bob travaille sur un projet propriétaire, les fruits de son travail bénéficient
à son employeur et dans une moindre mesure aux clients de ce dernier. Le jour où il change d'emploi, il perd tout le travail effectué
précédemment.

Quand Alice contribue ou travaille sur un projet open source, le fruit de son travail lui bénéficie à elle, aux
utilisateurs du projet open source et au reste de monde qui peut l'utiliser gratuitement, le modifier, l'améliorer.
Quand Alice change d'emploi ou le quitte, le résultat de son précédent travail lui reste accessible, elle peut
continuer à en bénéficier.

Ainsi les richesses produites se cumulent, plutôt que chaque acteur de la société gaspille des ressources afin de les recréer dans son coin.

<!-- Chaque entreprise, chaque organisation, n'a plus à tout réinventer dans son coin, elle est libre de s'appuyer si elle le veut
sur le travail effectué par d'autres pour y ajouter ce qu'elle estime être de la valeur. -->


#### Organisations distribuées

Les nouvelles technologies de communication (chat, visioconférence...) permettent aujourd'hui à des organisations et entreprises
de s'affranchir des bureaux classiques. Le personnel n'a plus besoin d'être dans la même ville, le même pays voire le même continent.


Cela pose de nouveaux défis au niveau des interactions sociales, mais c'est un premier levier pour
réduire notre  <a href="https://fr.wikipedia.org/wiki/%C3%89quation_de_Kaya" target="_blank" rel="noopener">consommation énergétique à l'échelle planétaire</a>.


<!-- Les membres n'ont plus besoin de tous se réunir sur un même lieu de travail pour finalement
rester la journée derrière un écran. -->


#### Coopération mondiale

Cette intersection des moyens de communication instantanée et de l'ouverture des outils de production permettent
une coopération globale. Les utilisateurs peuvent contribuer, participer à la création des produits qu'ils utilisent.
<!-- Ils ne sont plus simplement des consommateurs, mais acteurs, et ce sans barrière géographique. -->


Entrer dans un modèle de coopération est la seule solution pour limiter
la surexploitation de notre environnement et donc limiter notre propre empoisonnement.

<!-- **La coopération c'est avant tout un état d'esprit,
celui de créer et de profiter ensemble, que nous soyons employé
ou non.** -->


#### Résilience et durabilité

<a href="https://nest.com/whats-happening" target="_blank" rel="noopener">Que se passerait-il si demain Google perdait tout intérêt économique à faire des affaires et bloquait son accès à l'Europe ?</a>

<a href="https://techcrunch.com/2015/10/21/twitter-ceo-dorsey-apologizes-to-developers-says-he-wants-to-reset-relations" target="_blank" rel="noopener">Que se passerait-il si demain Twitter faisait faillite et stoppait ses services ?</a>

<a  href="https://theintercept.com/2016/05/02/whatsapp-used-by-100-million-brazilians-was-shut-down-nationwide-today-by-a-single-judge" target="_blank" rel="noopener"> Que se passerait-il si demain un gouvernement tyrannique bloquait WhatsApp ?</a>

La crise financière de 2008 nous a douloureusement rappelé que des entreprises que l'on croyait intouchables
peuvent faire faillite. Or, depuis <a href="https://fr.wikipedia.org/wiki/Dette_publique" target="_blank" rel="noopener">la situation économique mondiale</a> ne s'est pas améliorée.


L'open source permet aux projets de survivre non seulement à la faillite d'une entreprise, mais aussi à une coupure d'internet,
<a href="https://www.lemonde.fr/pixels/article/2019/05/20/google-coupe-les-ponts-avec-huawei-ce-que-ca-change-pour-les-utilisateurs_5464500_4408996.html" target="_blank" rel="noopener">à une guerre économique</a>.

<!--
Et puisque ces services entrent dans le bien commun, qu'ils ne sont pas la propriété d'une entrprise particulière,
ils deviennent beaucoup plus résilients à des tentatives de censure. -->


#### Un tremplin pour l'éducation

L'éducation façonne nos sociétés. Lui imposer des limites artificielles (*Copyright*, prix excessifs,
<a  href="https://fr.wikipedia.org/wiki/Gestion_des_droits_num%C3%A9riques" target="_blank" rel="noopener">DRM</a>...)
n'est que la garantie d'une mort lente et douloureuse de celles-ci.

L'Open source, quant à lui, profite triplement à l'éducation:
* En réduisant radicalement les coûts
* En améliorant la qualité des matériaux éducatifs
* En permettant d'étudier librement les systèmes existants


### Open data

L’open data *ou donnée ouverte est une donnée numérique dont l'accès et l'usage sont laissés libres.*


La donnée est aujourd'hui comparée au pétrole, de la même manière que ce dernier est le carburant de nos moteurs à
explosion, la donnée est le carburant des <a href="https://fr.wikipedia.org/wiki/Algorithme" target="_blank" rel="noopener">algorithmes</a>.

<!-- C'est en récoltant beaucoup de données et en leur donnant du sens que nous pourrons avoir des véhicules
autonomes et ainsi réduire drastiquement le nombre de morts sur les routes. -->
C'est en récoltant beaucoup de données et en leur donnant du sens
nous pouvons améliorer la détection de maladies, améliorer le recyclage, reduire notre consommation énergétique.

<!-- C'est grâce à la donnée que nous pouvons
optimiser des flux logistiques ou énergétiques et ainsi réduire notre trop grande consommation. -->


Le problème c'est qu'aujourd'hui ces données, produites par tout le monde, deviennent la propriété de grandes
entreprises (*les géants de la tech*), et sont utilisées entre autre pour <a href="https://fr.wikipedia.org/wiki/Cambridge_Analytica" target="_blank" rel="noopener">déstabiliser nos organisations politiques</a>.


Disposer des ces données en accès libre permettrait de multiplier la rapidité et la qualité de l'innovation dans tous domaines.
<!-- de la robotique, de la médecine, des transports, de l'agriculture etc... mais surtout de grandement réduire les besoins en resource dans tous ces domaines. -->

À l'heure où nous déléguons de plus en plus notre mémoire et notre savoir à des entreprises qui les enferment dans des silos privés, nous n'avons
absolument aucune garantie quant à leur avenir, tant au niveau de leur préservation qu'au niveau
de leur intégrité.


### Open Access

*L’open access (ou aussi « libre accès », ou encore « accès ouvert ») à la littérature scientifique est un mode de diffusion des articles de recherche sous forme numérique, gratuite et dans le respect du droit d’auteur.*

<a href="https://www.echosciences-grenoble.fr/articles/la-controverse-en-matiere-de-publication-des-articles-scientifiques" target="_blank" rel="noopener">Le monde de la publication scientifique est gangrené</a>.

Il n'y a rien de plus à dire: tout le monde s'accorde sur l'intérêt pour le bien commun de publier publiquement et gratuitement les résultats
des chercheurs financés aussi bien par le secteur public que privé (<a href="https://www.service-public.fr/professionnels-entreprises/vosdroits/F23533" target="_blank" rel="noopener">ce dernier étant de toute façon financé par nos impôts</a>...).

Je recommande le livre <a href="http://ia601805.us.archive.org/17/items/9780262517638OpenAccess/9780262517638_Open_Access.pdf" target="_blank" rel="noopener">Open Acces de Peter Suber</a> pour approfondir le sujet.


### Transparence : vertueuse mais surtout nécessaire

Toutes ces mouvances d'ouverture apportent de fait de la transparence. L'information devient accessible.
N'importe qui, sans distinction aucune, peut s'en saisir, l'enrichir, lui donner du sens.

Mais cette transparence vertueuse est surtout nécessaire, car si les vraies informations se propagent
à la vitesse de la lumière, les fausses aussi (*fake news*).
<!-- Aujourd'hui mises en avant car elles menacent la stabilité
de nos sociétés,  -->
Il existe 2 grandes directions pour restreindre leurs effets :

* La censure de masse
* La transparence

D'un côté il est facile de comprendre pourquoi la censure de masse est néfaste, mais de l'autre côté la transparence n'est pas suffisante à elle seule pour mettre fin aux fausses informations. Elle permet avant tout d'apporter des faits contradictoires et prouvables (contrairement aux *fake news*).


La première étape de la transparence est de rendre l'information publique. La seconde est de la rendre facile
d'accès, facile à chercher, facile à réutiliser.

Sans transparence, il n'y a pas de vie publique, pas de confiance possible, société stable. La transparence
est le fondement même du contre-pouvoir.
<!-- Sans transparence le *lobbying* ne doit être considéré que comme de la corruption. -->


## Bloom : le master plan top secret (Quoi ?)


<!-- Nous croyons en un changement de paradigme,
dans la manière dont nous consommons mais surtout dans la manière dont nous produisons, -->
**Nous croyons en un monde de partage et de coopération**. C'est pourquoi nous avons créé
<a href="https://bloom.sh" target="_blank" rel="noopener">Bloom</a>.

<div class="center">
  <img src="/2019/bloom-a-free-and-open-source-google/blog.png" height="400"/>
</div>

<!-- <a href="https://bloom.sh" target="_blank" rel="noopener">Bloom</a>
se veut l'amorce d'un changement de paradigme, dans la manière dont nous consommons mais surtout dans
la manière dont nous produisons. -->

<!-- Un monde de collaboration plutôt que de compétition. Nous n'avons simplement pas le luxe de pouvoir continuer à concourir pour les mêmes
resources finies, faisant fi des destructions et pollutions engendrées. -->


Bloom est une organisation qui grâce à l'Open source/access/data..., redistribue gratuitement et librement
sa production à large échelle, en s'appuyant sur les nouvelles technologies pour le faire à moindre
coût plutôt que de les utiliser comme outil impérialiste. L'éthique plutôt que le profit, même si ce dernier
n'est pas négligé afin d'assurer la longévité de l'organisation.
<!-- Nous voulons briser la frontière entre entreprises et consommateurs. -->

Ce nouveau paradgime permet de réduire considérablement notre impact écologique à l'échelle globale puisque
le reste du monde n'a pas à gaspiller des ressources et de l'énergie pour réinventer
ce que nous (ou d'autres) avons déjà développé.


C'est ainsi un cercle vertueux qui se forme, où la coopération brise la frontière entre entreprises et consommateurs.
Où production ne rime plus avec auto-destruction.


Notre mission ?
**Faire de notre planète un meilleur endroit pour tout le monde grâce aux technologies ouvertes.**

<!-- C'est à nous de choisir aujourd'hui si nous voulons subir un monde où l'on doit travailler toujours plus pour entretenir et exacerber des inégalités déjà criantes,
ou si nous voulons d'un monde où les technologies sont au service du bien de tous. Cet avenir n'est pas encore
écrit. -->

Bloom propose aujourd'hui les services en ligne suivants :

* <a href="https://bloom.sh/drive" target="_blank" rel="noopener">Bloom Drive</a>: un espace de stockage
* <a href="https://bloom.sh/platform/phaser" target="_blank" rel="noopener">Phaser</a>: un scanner de sécurité automatique pour sites webs
* <a href="https://bloom.sh/bitflow" target="_blank" rel="noopener">Bitflow</a>: un gestionnaire de téléchargements, qui permet de télécharger des fichiers directement dans votre *drive* (torrents, HTTP).
* <a href="https://bloom.sh/music" target="_blank" rel="noopener">Bloom Music</a>: un lecteur de musique lisant les musiques directement depuis votre *drive*
* <a href="https://bloom.sh/gallery" target="_blank" rel="noopener">Bloom Gallery</a>: une galerie pour vos photos et vidéos
* <a href="https://bloom.sh/contacts" target="_blank" rel="noopener">Bloom Contacts</a>: un gestionnaire de contacts
* <a href="https://bloom.sh/notes" target="_blank" rel="noopener">Bloom Notes</a>: une application de prise de notes

<!-- Une application Android est disponible :
<a href="https://play.google.com/store/apps/details?id=com.bloom42.bloomx" target="_blank" rel="noopener">https://play.google.com/store/apps/details?id=com.bloom42.bloomx</a> -->

Tous ces services offrent des tiers gratuits (ou sont entièrement gratuits) sans contrepartie (pas de ciblage, pas de publicité...),
et dont le code source est disponible librement sur <a href="https://github.com/skerkour/bloom" target="_blank" rel="noopener">GitHub</a>.

Aujourd'hui orientés productivité nous contribuerons demain
à des secteurs plus divers tels que l'agriculture, l'architecture, le transport...

**Bloom est le Linux du XXIe siècle.**

Bloom concilie le meilleur de la **religion Startuplaland** : *l'exécution radicale
plutôt que la bureaucratie, la scalabilité, une expérience utilisateur sans faille, une culture forte pour le produit, un processus itératif*,
et de l'**open source**: *un mode de développement qui a prouvé sa supériorité, une gouvernance ouverte, une durabilité inégalée
et une symbiose parfaite des intérêts des développeurs et de ceux des utilisateurs.*


Nous avons choisi la <a href="https://fr.wikipedia.org/wiki/Copyleft" target="_blank" rel="noopener">license virale</a>
<a href="https://www.gnu.org/licenses/agpl-3.0.fr.html" target="_blank" rel="noopener">AGPL v3.0</a> pour la plupart de nos créations afin d'encourager d'autres personnes à contribuer en retour et
à faire de même. **Pas d'open core, pas d'entourloupe**.


Concernant la gouvernance, la fédération est sur la feuille de route, mais nous devons d'abord trouver des solutions
aux problèmes affectant les systèmes existants (spams dans le monde des emails, arbitrages législatifs dans le monde de Mastodon...).

<!-- Nous n'avons pas encore de réponse concernant la gouvernance mais nous réfléchissons à un système
de ouvert, multi-latéral tout en préservant la sécurité et l'intergrité du système
(ce que ne permet pas par exemple la fédération simple, comme le montre les spams dans le monde des emails, les contenus
illégaux dans le monde de Mastodon ou encore les attaques 51% contre les *blockchains*).
Nous sommes ouverts à toute discussion, proposition sur le sujet. -->


### Contribuer

Nous croyons en un monde où chacun peut contribuer, pas seulement des experts.
**Où citoyens et consommateurs deviennent acteurs.**
C'est pourquoi nous sommes engagés à faciliter cette coopération mondiale et sans barrières.



#### Communication

La langue officielle de communication de Bloom est l'anglais.

<!-- Un chat Discord est disponible pour demander de l'aide à la communauté: <a href="https://discord.gg/HmDQDGv" target="_blank" rel="noopener">discord.gg/HmDQDGv</a>. -->


Enfin, pour rester informé des dernières mises à jour suivez nous sur Twitter <a href="https://twitter.com/sylvain_kerkour" target="_blank" rel="noopener">@sylvain_kerkour</a> et sur Mastodon: <a href="https://mastodon.social/web/accounts/947209" target="_blank" rel="noopener">@42bloom@mastodon.social </a>.


#### Développement

Pour contribuer au code cela se passe sur <a href="https://github.com/skerkour/bloom" target="_blank" rel="noopener">github.com/skerkour/bloom</a>.

Plus d'informations est disponible sur notre site <a href="https://bloom.sh/open-source" target="_blank" rel="noopener">bloom.sh/open-source</a>.

##### Rust

<a href="https://www.rust-lang.org/" target="_blank" rel="noopener">Rust</a> est le langage de programmation officiel de Bloom, pour une multitude de raisons, les principales
étant :

* Un langage expressif et moderne
* Un langage open-source et embrassant pleinement nos valeurs
* Un langage permettant à la fois des abstractions de haut niveau et des constructions de bas niveau, sans coût de performance
* Tous nos services doivent pouvoir être auto-hébergés sur un <a href="https://www.raspberrypi.org" target="_blank" rel="noopener">Raspberry pi</a>
* Là encore les intérêts composés entrent en jeux, être expert dans un langage capable à la fois de faire des services web,
de la robotique ou encore des blockchains, permet de multiplier les capacités de création
* <a href="https://insights.stackoverflow.com/survey/2019" target="_blank" rel="noopener">Les développeurs l'aiment <3</a>
* Car c'est <a href="http://www.paulgraham.com/avg.html" target="_blank" rel="noopener">LE langage pour les gouverner tous</a>


Il sera utilisé pour **tous** les projets de Bloom sauf impératif contraire
(Typescript + VueJS pour les applications web, Kotlin pour les applications Android, Swift pour les applications IOS...).


#### Design

Pour contribuer au design de Bloom cela se passe sur le repo <a href="https://github.com/skerkour/bloom" target="_blank" rel="noopener">github.com/skerkour/bloom</a>.


#### En partageant

Diffusez les idées, débattez avec votre entourage, partagez Bloom sur les réseaux sociaux ! Nous ne sommes encore qu'au début de l'aventure et nous faire connaître nous aide ééénooormément !


#### Financièrement

Si vous n'avez pas le temps ou les compétences pour contribuer, mais que notre projet vous semble prometteur
vous pouvez contribuer financièrement.

Cet argent sera utilisé pour financer les personnes contribuant avec leur temps à un monde plus ouvert :

* En souscrivant à un abonnement sur <a href="https://bloom.sh" target="_blank" rel="noopener">bloom.sh</a>.


Un grand merci aux personnes ayant déjà contribué directement ou indirectement :

* Ma famille
* Marina Da Silva
* L'école 42
* Jean-Roger Callière
* Katerina Limpitsouni
* Nikolay Kim
* Sean Griffin
* Ed Page
* Jesse Szwedko
* "mdouchement"
* "erickeller"
* Dmitriy Olshevskiy
* Tyson Clugg
* Ben Overmyer
* "pulsonics"
* Jose Nazario
* Andrew Gallant


Ainsi qu'à :
* Pablo Servigne
* Cécile Serrano
* Josette Crastes
* Étienne Théodore
* Les "K" de Music'All
* Yanaël Barbier
* Nabil Brizini


pour la relecture,

Et à tous les autres enthousiastes du partage sans qui cela n'aurait pas été possible.

## En résumé

En résumé le master plan est le suivant :

1. Créer des logiciels libres et faire payer l'hébergement, la sécurité des données hébergées et le support entreprise<br />
2. Avec cet argent réduire les prix, libérer la donnée et l'accès à la connaissance scientifique<br />
3. Avec cet argent et cette communauté créer l'infrastructure ouverte pour faire fonctionner ces logiciels et héberger ces données ouvertes

**Notre mission ?** Faire de notre planète un meilleur endroit pour tout le monde grâce aux technologies ouvertes.

**Nos valeurs ?** Coopération, Acuité, Transparence, Simplicité (**CATS** 🐱).

**Chronologie :**
* 2018 - Échec de trouver <a href="/the-just-cause-and-the-infinite-game/" target="_blank" rel="noopener">LE *pourquoi ?*</a>
* Q1/Q2 2019 - <a href="https://bloom.sh" target="_blank" rel="noopener">Beta</a>
* Q3 2019 - <a href="https://www.inkandswitch.com/local-first.html" target="_blank" rel="noopener">Offline first</a> + auto-hébergement
* Q4 2019 - <a href="https://fr.wikipedia.org/wiki/Chiffrement_de_bout_en_bout" target="_blank" rel="noopener">Chiffrement de bout en bout</a> + teams
* Q1 2020 - Ajout de services
* Q2 2020 - Application IOS + Portabilité des données
* Q3/Q4 2020 - Fédération
* ∞ Ne jamais arrêter de défier le *status quo*

Mais chuuut, c'est un secret 🤫

Contact : <span id="email">Javascript is required to access email address</span>

-------

*Pour triompher, le mal n’a besoin que de l’inaction des gens de bien...*

♪ <a href="https://www.youtube.com/watch?v=dERx6uFUf6o" target="_blank" rel="noopener">MC Solaar - Sauvez le monde</a>

📖 <a href="https://www.ynharari.com/fr/book/21-lessons" target="_blank" rel="noopener">21 leçons pour le XXIe siècle, Yuval Noah Harari</a>

🎬 <a href="https://www.youtube.com/watch?v=4yXKCnz9lRc" target="_blank" rel="noopener">Internet ou la révolution du partage | ARTE</a>


<script type="text/javascript">
  window.addEventListener("load", function(){
    var email = document.getElementById("email");
    if (email) {
        email.innerHTML = rot13('<n uers="znvygb:uryyb@oybbz.fu">uryyb@oybbz.fu</n>');
    }
  });
  function rot13(s) {
    return (s ? s : this).split('').map(function(_){
      if (!_.match(/[A-Za-z]/)) return _;
      c = Math.floor(_.charCodeAt(0) / 97);
      k = (_.toLowerCase().charCodeAt(0) - 83) % 26 || 26;
      return String.fromCharCode(k + ((c == 0) ? 64 : 96));
    }).join('');
  }
</script>
