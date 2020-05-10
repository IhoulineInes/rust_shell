Ines Ihouline 

# **Rust shell**
---------------


### 1 - Introduction

Bienvenue dans la suite du cours de programmation systèmes et réseaux. Au travers ce cours, nous allons visiter des concepts liés aux capacités qu’offrent votre hardware et vos systèmes d’exploitations, afin de vous dévoiler la magie qui est cachée par les langages de haut niveau.
L’étendue des concepts est très vaste, nous ne pourront pas tout voir malheureusement.

#### 1.1 Processus
Au cours de ce travail pratique nous allons commencer par démystifier l’abstraction des processus en réalisant un mini invité de commande très simple.
Que vous soyez sur Linux, MacOS ou Windows vos programmes sont isolés les un des autres, cette abstraction s’appelle le « processus », parfois appelée «tâche». Ce mécanisme de base permet d’implémenter des isolations plus fortes comme celles utilisées dans docker ou les machines virtuelles
type QEMU.
Mais dans la vie de tout les jours les processus permettent d’écrire des programmes sans avoir accès aux autres programmes sur l’ordinateur.
Attention : Nous reverrons plus tard un concept proche déjà vu au premier semestre, le concept de thread ou fil de calcul. Un thread est un fil de calcul en plus dans un processus ! Pour avoir un autre programme il faut faire un processus, si vous voulez juste faire des calculs sur un autre processeur, un thread est ce qu’il vous faut !
Un processus possède son propre espace d’adressage découpé en segments, c’est à dire sa propre pile stack, son propre tas heap, son propre code dans le segment .text, ses propres données connues à la compilation .data et d’autres avec leur propre usage comme l’espace des bibliothèques dynamiques.
La taille de cet espace dépend de votre système, sur un système 32bit cet espace va de l’adresse 0x0000_0000 à 0xFFFF_FFFF sur 64 bits de 0x0000_0000_0000_0000 à 0xFFFF_FFFF_FFFF_FFFF 1 la formule pour connaître l’espace maximum d’adressage est la suivante : 2 n − 1.
Cet espace est découpé comme nous l’avons vu avant en segments, c’est votre système d’exploitation qui orchestre cette abstraction, les compilateurs lieurs linkers et assembleurs la respectent.

#### 1.2 Questions : Rappels de Rust, généralités

1. En Rust à quoi servent les références ?

> Les références permettent un accès multiples à la même variables en lecture et/ou en écriture. Elles ont une durée de vie qui dépend de leur portée, une fois que cette portée dépassée, l'emprunt n'est plus valable et est donc retiré de la mémoire.


2. Citez en Rust les grandes façons de déclarer ses propres types.

> Les grandes façon de déclarer ses propres types en Rust sont: Enum et Struct, bien que nous pouvons aussi se servir d'alias pour déclarer d'autre type.

3. Rust est compilé nativement (assembleur sous forme de code machine) ou compte sur une machine virtuelle pour s’exécuter ?

>	Rust se compile nativement grâce à LLVM, une machine virtuelle.


4. Imaginons qu’on a un système avec un processeur 8bits, quelle est la valeur maximale adressable ?
Écrire la solution en notation hexadécimale et décimale.

> Si on a un système avec un processeur 8bits, 2^8 - 1 = 255, soit en hexadécimal FF

5. Donnez votre définition d’un processus citez vos sources !

>	Un processus est comme un conteneur qui permet d’exécuter, de stocker et isoler des programmes en cours.


### 2 - Pratique - micro-shell

#### 2.1 Questions : Deployement du projet et entrées sorties

1. Comment compiler puis exécuter son programme ? Exécuter les test ? Où sont rangés les binaires (en mode debug) ?

> Pour executer son programme il faut faire un cargo build et pour executer cargo run. Pour executer les tests il faut faire un cargo test.
Les binaires en mode debug sont dans le dossier target/debug/build.


### 3 - Execution d’un Processus

#### 3.1 Questions : Executer une commande

3. Réussir à exécuter une commande avec std::process::Command::status.

4. Afficher le statut d’une commande, pourquoi Rust vous force à récupérer le statut

> Rust suppose que si principal programme revient, c'est qu'il s'est terminé avec succès. Ce n'est qu'en appelant explicitement des fonctions comme expect ou std :: process :: exit que le programme peut se terminer avec un code d'état d'erreur. C'est pour cela que le programme attend qu'il se terminer pour collecter son statut afin de savoir quoi faire.

5. Que fait votre programme pendant que son enfant s’exécute
>Pendant que son enfant s'exécute, le programme attend qu'il se termine.


6. Maintenant gérer mais pour une commande avec plusieurs arguments !



### 4 - Redirections - pipe my programs’

#### 4.1 Questions : Redirections

7. Donnez avec vos mot une définition d’un tupe entre deux programmes citez vos sources.
>Un tube entre deux programmes crée une liaison, un dialogue entre la sortie des deux programme.

8. Écrire une version basique ou une commande sans argument traite une commande sans arguments par exemple un simple ls redirigé dans le programme echo.


9. Écrire une version plus avancée où nos deux programmes s’exécutent vraiment en même temps (pas de triche avec std::process::Command::output) et gèrent plusieurs arguments aux deux commandes.

### 5 - Executions en concurence : Gérer des commandes en fond

#### 5.1 Questions

10. C’est quoi un processus id ? Citez vos sources.
>processus id est un entier qui représente un identifiant unique faisant référence à un programme lors de son lancement, c'est à dire à un processus.


11. Écrire une implémentation basique qui gère un seul job en tâche de fond.


12. Écrire une structure 8 pour stocker les programmes en cours d’exécution de votre shell.
>

13. Écrire une commande jobs qui affiche les programmes en cours d’exécution en fond et leur état.
>
