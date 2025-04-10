# 🐾 TP - Système d'Élevage Pokémon (Langage Rust)

## 📚 Contexte

Ce projet a pour but de développer un système de gestion d’élevage de Pokémon en Rust. Il faut modéliser des Pokémon, suivre leur évolution, gérer leur expérience, et simuler leur reproduction selon certaines règles logiques.

Ce projet  permet de mettre en pratique des concepts fondamentaux en Rust, tels que les structures (`struct`), les énumérations (`enum`), la gestion d’état d’un objet, et l’utilisation de collections.

---

## 🎯 Objectifs pédagogiques

- Manipuler des structures de données (`struct`, `enum`)
- Comprendre la gestion de l'état d'un objet (niveau, expérience…)
- Utiliser les collections comme les vecteurs (`Vec`)
- Appliquer une logique métier conditionnelle

---

## 🧱 Structure du projet

### Partie 1 : Définir les Pokémon

- Créer une énumération `TypePokemon` représentant les types : `Feu`, `Eau`, `Plante`, `Electrik`, etc.
- Créer une structure `Pokemon` contenant :
  - `nom` (String)
  - `niveau` (u32)
  - `type_pokemon` (TypePokemon)
  - `experience` (u32)
  - `genre` (enum Genre : Male ou Femelle)

---

### Partie 2 : Fonctions et comportements

- **Gagner de l'expérience (XP)**  
  Lorsqu’un Pokémon atteint 100 XP, il gagne un niveau.

- **Affichage d’un Pokémon**  
  Fonction pour afficher toutes les informations d’un Pokémon.

- **Vérification de reproduction**  
  Deux Pokémon peuvent se reproduire s’ils :
  - Ont le **même type**
  - Sont de **genres opposés**
  - Ont un **niveau suffisant**

- **Reproduction**  
  Une fonction permet de tenter une reproduction entre deux Pokémon compatibles. Si elle réussit, un nouveau Pokémon est créé :
  - Niveau : 1
  - Type : même que les parents
  - Nom : `"Mystère"`

---

### Partie 3 : Gestion de l'élevage

Créer une structure `Elevage` permettant de gérer plusieurs Pokémon. Cette structure permet :

- Ajouter un nouveau Pokémon
- Afficher tous les Pokémon de l’élevage
- Entraîner tous les Pokémon (gain d’XP)
- Tenter une reproduction entre deux Pokémon de l’élevage

---

### 🌟 Bonus (facultatif)

- Générer aléatoirement :
  - Le **genre** du nouveau Pokémon
  - Le **nom** du nouveau Pokémon
- Trier les Pokémon :
  - Par **niveau**
  - Par **type**
- Sauvegarder et charger les données depuis un fichier (avancé)

---

## 🧾 Lexique

| Terme   | Description |
|--------|-------------|
| **XP** | Points d’expérience gagnés par un Pokémon. Chaque tranche de 100 XP fait monter d’un niveau. |
| **Niveau** | Indique la puissance d’un Pokémon. Tous commencent au niveau 1. |
| **Type** | Élément de base d’un Pokémon (Feu, Eau, etc.). |
| **Genre** | Sexe d’un Pokémon : Male ou Femelle. |

---

## 🚀 Technologies

- Langage : [Rust](https://www.rust-lang.org/)
- Concepts : Struct, Enum, Vecteurs, Traitements conditionnels, Match, Implémentation de méthodes

---


## 💬 Auteur·e

**Kenza SCHULER M2 IW**
Projet réalisé dans le cadre d’un TP d’Ingénierie du Web (M2) à l’ESGI.  
Développé avec ❤️ en Rust.

---