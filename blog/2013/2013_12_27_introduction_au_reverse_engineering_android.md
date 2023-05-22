+++
date = 2013-12-27T12:42:42+02:00
title = "Introduction au reverse engineering - Android"
authors = ["Sylvain Kerkour"]
type = "post"
tags = ["hacking", "french"]
url = "/introduction-au-reverse-engineering-android"


[extra]
lang = "fr"
+++

1. [Introduction](#introduction)
2. [APKOA](#apkoa)
3. [Les outils](#les-outils)
4. [GO GO GO](#go-go-go)
5. [Bingo !](#bingo)
6. [Plus d’infos](#plus-d-infos)

<hr />

## Introduction

Le __reverse engineering__, c’est ( en gros ) comprendre le fonctionnement d’un
dispositif, par une analyse en profondeur. Comprendre comment les concepteurs ont
imaginé le système.

Cela peut servir dans le cadre d’une séance de debugging d’un logiciel dont on a
pas les sources, trouver des failles de sécurité ou, ce que l‘on va faire ici,
modifier un programme ( dont on a pas les sources ) et ses fonctionnalités pour
l’adapter à nos envies.

On le verra plus tard, lorsque l’on étudiera les plateformes x86, en reverse
engineering, quand on fait ça pour le fun, il s’agit souvent de faire en sorte
de sauter une routine de vérification ( par exemple *- un pur hasard -* ,
vérification si l’on a une licence payante ).

Il est donc recommandé d’avoir quelques notions de programmation, ou à défaut,
d’avoir un peu d’instinct.

Cela peut aussi être utile pour récupérer les assets d’une application.



## APKOA ?

Sous android, on distribue les programmes sous forme d’APK : ce sont simplement
des archives zip signées et renommées.

Pour l’exemple, on va prendre l’APK de plagueInc. Il s’agit d’un merveilleux jeu
de stratégie. Comme beaucoup (trop) de jeux sur ce système d’exploitation, il y
a une version gratuite, avec quelques malus et une version payante, complète.
Pourquoi ce jeu ? Car il est bien, et car il ne présente pas de difficultés,
il convient donc parfaitement en guise d’introduction.

Pour ne pas porter préjudice aux développeurs qui ont mis du temps à faire un
( très ) bon jeu, on ne va pas débloquer comme par magie les bonus payants, mais,
juste faire en sorte de virer le panneau publicitaire ( puisque il existe des
applis android qui permettent de virer les pubs ) qui gâche un peu l’esthétique
du jeu. On va aussi modifier les permissions de l’application car celles demandées
de base semblent un peu trop intrusives.

<p>
<img src="/2013/introduction-au-reverse-engineering-android/permissions_original.jpg" width="240" height="400" style="display: inline-block;"/>
  <img src="/2013/introduction-au-reverse-engineering-android/menu_pubs.jpg" width="240" height="400" style="display: inline-block;"/>
  <img src="/2013/introduction-au-reverse-engineering-android/game_pubs.jpg" width="240" height="400" style="display: inline-block;"/>
</p>

Il existe principalement 2 méthodes pour étudier des applications Android :

1. Décompiler les sources (avec Java Decompiler parexemple), cela permet une lecture
seule, mais on obtient directement du Java.

2. Désassembler le code, on obtient le bytecode Dalvik, on peut ainsi le modifier,
et réassembler l’application par la suite. C’est ce que nous allons faire.



## Les outils
On va avoir besoin de plusieurs outils :

* Le kit de développement pour android : http://developer.android.com/sdk/index.html.
Pour ma part j’ai pris le Linux-32bits.
Il va falloir ajouter à votre `PATH` l’outil `aapt`.
Cela peut être fait, par exemple, en ajoutant à la fin de votre fichier `.bashrc` :

```bash
export PATH=$PATH:/home/xxx/reverse/android/adt-bundle-linux-x86–20131030/sdk/build-tools/android-4.4
export PATH=$PATH:/home/xxx/reverse/android/adt-bundle-linux-x86–20131030/sdk/platform-tools
```

En adaptant évidemment les dossiers.
Quand vous entrez `aapt` dans votre console, il doit y avoir un retour.

* Java : pour pouvoir utiliser les outils, j’utilise l’open jdk 6, mais le 7
fonctionne aussi; un `java -version` me renvoie :
```bash
java version '1.6.0_27'OpenJDK Runtime Environment (IcedTea6 1.12.6) (6b27–1.12.6–1~deb7u1)
```
* Android-apktool l’outil principal : c’est celui qui va nous permettre de
décompiler les .apk.
* keytool : afin de faire en sorte de pouvoir signer nos applications modifiées.
* Une référence quant aux OPcodes Dalvik afin de comprendre ce qu’on va lire.



### GO GO GO
On se place dans notre dossier de travail créé pour l’occasion et on commence par créer le fichier qui nous permettra de signer nos applications :
keytool -genkey -v -keystore noob.keystore -alias noob -keyalg RSA -keysize 2048 -validity 10000
Je vous laisse remplir les champs comme bon vous semble.

Ensuite, il faut récupérer l’APK, pour cela 2 méthodes

1. en passant par le SDK android : Télécharger l’application sur l’android market depuis l’appareil, puis on le connecte à l’ordinateur,
sudo adb start-serveradb devices
cela doit vous renvoyer une sorte de numéro de série pour bien vous indiquer qu’un appareil est attaché ( bien penser à activer le débogage USB sur votre appareil android ).
```bash
adb shell ls /data/app | grep plague
```
doit renvoyer quelque chose du genre :
```bash
com.miniclip.plagueinc-1.apk
```
En effet, la plupart des applications installées dans la mémoire flash des
appareils android se situent dans ce dossier `/data/app` . On récupère l’APK donccomme cela :
```bash
adb pull /data/app/com.miniclip.plagueinc-1.apk
```
qui charge l’application dans le répertoire courant.

2. Sinon, on peut utiliser ce service : https://apps.evozi.com/apk-downloader/ qui permet de télécharger directement l’APK sur notre ordinateur.


On doit maintenant avoir dans notre dossier de travail : `apktool.jar` et `com.miniclip.plagueinc-1.apk`.
on lance :
```bash
java -jar apktool.jar d com.miniclip.plagueinc-1.apk apk
```
On utilise apktool pour dépacker l’archive du jeu, et mettre le tout dans un nouveau dossier apk.


On se déplace dans ce dossier. Si vous avez déjà un peu programmé pour android, l’arborescence sous vos yeaux doit vous être un familière : on a ( entre autre ) un dossier assets où l’on peut modifier les ressources de l’application, un dossier res avec notamment les string utilisées, l’AndroidManifest, et un dossier smali, qui nous intéressera plus tard, il contient le code désasemblé.


Pour commencer, on va virer les permissions qui semblent inutiles (/!\ cela ne fonctionne pas tout le temps avec toutes les applications, cela peut les faire crasher).
on ouvre l’`AndroidManifest.xml`, on descend dans le fichier et on modifie pour restreindre les permissions :
on transforme :
```xml
uses-permission android:name="android.permission.INTERNET"
uses-permission android:name="android.permission.ACCESS_NETWORK_STATE"
uses-permission android:name="android.permission.READ_PHONE_STATE"
uses-permission android:name="android.permission.ACCESS_WIFI_STATE"
uses-permission android:name="com.android.vending.BILLING"
uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"
uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION"
```
en
```xml
uses-permission android:name="android.permission.INTERNET"
uses-permission android:name="android.permission.ACCESS_NETWORK_STATE"
uses-permission android:name="android.permission.ACCESS_WIFI_STATE"
uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"
```
( ici j’ai enlevé les ouvertures et fermeture des balises pour la simplicité de publication )


Voilà une bonne chose de faite. On entre maintenant dans la partie ardue mais surtout, la plus intéressante pour notre esprit d’enquêteur :
comprendre et modifier le comportement de l’application. Ce qu’on veut c’est enlever les pubs, grâce à nos super $k1llz d’english language, on sait que pubs se traduits par ads. Un
```bash
grep -r "Ads" *
```
nous renvoie des résultats qui peuvent sembler extravagants mais qui en faits ne le sont pas tant que ça. On note de nombreuses références au fichier `smali/com/miniclip/nativeJNI/cocojava.smali`.
Notre curiosité nous pousse donc à l’ouvrir avec notre éditeur de texte préféré :
```bash
vim smali/com/miniclip/nativeJNI/cocojava.smali
```
Oo un peu plus de 13 000 lignes … je vous rassure on ne va pas toutes les lire.

On lit les références en haut du fichier, et une attire notre attention :
inAppsRemoveAds. Tiens, ça semble être ce qu’on veut.
c’est parti pour un
```bash
grep -r "inAppsRemoveAds" *
```
qui nous renvoie
```bash
smali/com/miniclip/nativeJNI/cocojava.smali:.field private static inAppsRemoveAds:Z
smali/com/miniclip/nativeJNI/cocojava.smali: sput-boolean v2, Lcom/miniclip/nativeJNI/cocojava;->inAppsRemoveAds:Z
smali/com/miniclip/nativeJNI/cocojava.smali: sput-boolean p3, Lcom/miniclip/nativeJNI/cocojava;->inAppsRemoveAds:Z
smali/com/miniclip/nativeJNI/cocojava.smali: sput-boolean p3, Lcom/miniclip/nativeJNI/cocojava;->inAppsRemoveAds:Z
smali/com/miniclip/nativeJNI/cocojava.smali: sget-boolean v1, Lcom/miniclip/nativeJNI/cocojava;->inAppsRemoveAds:Z
```
Décidément, ce fichier semble vraiment intéressant.


Grâce notre lecture approfondie de la doc, on sait que dans ces résultats, c’est surtout la dernière ligne qui nous intéresse. En effet, c’est là que le programme appellera une variable pour la comparer. Ça ressemble fortement à une routine de vérification, le genre de truc qu’on aime ;) .
On recherche donc cette chaîne de caractères ( / avec vim, on cherche sget-boolean v1, Lcom/miniclip/nativeJNI/cocojava;->inAppsRemoveAds:Z ), ce qui nous amène à la ligne 11296 de notre fichier cocojava.smali ( indice : .ligne 3026 ).

Tiens juste en dessous, un saut conditionnel, qui si il est pris, saute la méthode getAdsDisabled. On va donc voir ce que fait cette méthode ( chercher getAdsDisabled ), ce qui nous amène à la ligne 3245 de notre fichier `cocojava.smali` ( indice : .ligne 2952 ). On voit que cette méthode est surchargée, la première, pas très intéressante, sans arguments appelle la seconde avec un argument.

Tiens, on peut voir à la ligne 3267 ( dans la fonction getAdsDisabled avec un argument ) la chaîne ADS_DISABLED. On semble sur la bonne voie !
On voit dans cette méthode, que le seul saut conditionnel est vers la fin :
```smali
.line 2946
.local v0, adsDisabled:Ljava/lang/String;
const-string v3, "true"
invoke-virtual {v0, v3}, Ljava/lang/String;->equals(Ljava/lang/Object;)Z
move-result v3
if-eqz v3, :cond_0
Bon, ça semble un peu simple, mais pourquoi ne pas tenter le coup ? on transforme donc cette portion en :
.line 2946
.local v0, adsDisabled:Ljava/lang/String;
const-string v3, "true"
const-string v0, "true"
invoke-virtual {v0, v3}, Ljava/lang/String;->equals(Ljava/lang/Object;)Z
move-result v3
if-eqz v3, :cond_0
```
On va mainteant reconstruire l’apk pour voir si ça fonctionne :

on se replace dans notre répertoir de travail initial et on lance

```bash
java -jar apktool.jar b apk/ cracked.apk && jarsigner -verbose -keystore noob.keystore -digestalg SHA1 -sigalg MD5withRSA cracked.apk noob
```

Ce qu’on fait ici : on reconstruit une archive nomée `cracked.apk` à partir des fichiers contenus dans le dossier apk (là où sont nos fichiers issus de l’apk original ), et ensuite on signe l’archive avec le certificat créé au début.

Pour installer l’application, on peut soit faire
```bash
adb install cracked.apk
```
soit, si vous avez des problèmes avec vos drivers, mettre l’apk sur un support
auquel vous pourrez avoir un accès avec par exemple , es file explorer .
Pensez à supprimer l’application initiale, car il peut y avoir des conflits étant
donné que l’on a signé nous même l’application modifiée, vous aurez donc deux
applications avec le même nom, mais avec des signatures différentes.



## Bingo !
et … TADDAA, ça fonctionne : plus de bannière de pub, et plus de permissions
inutiles( attention tout de même sur ce point, cela peut mener à des crashs si
  cela est mal fait).

<p>
  <img src="/2013/introduction-au-reverse-engineering-android/permissions_mod.jpg" width="240" height="400" style="display: inline-block;"/>
  <img src="/2013/introduction-au-reverse-engineering-android/menu_mod.jpg" width="240" height="400" style="display: inline-block;"/>
  <img src="/2013/introduction-au-reverse-engineering-android/game_mod.jpg" width="240" height="400" style="display: inline-block;"/>
</p>


Gardez bien en tête que les gens qui font des applications y ont passé du temps,
et si ils jugent mériter salaire pour le travail accompli c’est que c’est peut
être le cas.


## Plus d’infos :
* http://virtualabs.fr/Reversing-d-applications-Android
* http://pallergabor.uw.hu/androidblog/dalvik_opcodes.html
* http://lifehacker.com/5994516/how-to-take-a-screenshot-on-android
* http://fr.openclassrooms.com/informatique/cours/creez-des-applications-pour-android
* http://developer.android.com/tools/help/adb.html
