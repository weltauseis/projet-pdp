# Time Curves

## Organisation du projet :

Oubliez pas qu'on devra rendre ce repo au chercheur(s) qui nous corrigeront donc pas de commits bizarres et un workflow un minimum sérieux svp 🙏🙏🙏

🚨 Commitez absolument du code formatté (`shift + alt + f` sur vscode) pour éviter de devoir faire des commits de formatage qui polluent le repo 🚨

## Structure :

### Backend : 

Prend en entrée des données et produit des courbes : composé d'une implémentation dans `lib.rs` et d'un programme en ligne de commande dans `main.rs`.

Il doit avoir une architecture qui permet facilement d'échanger les méthodes utilisées, voir le design pattern `Method Template` et `Strategy`. Par exemple, l'implémentation du papier utilise la méthode de MDS dite *classical*, mais il mentionne aussi la méthode ISOMAP.

Le fichier d'entrée est censé contenir une matrice de distance, mais il faudrait aussi pouvoir la générer en fonction du type de données d'entrée. Plusieurs méthodes sont données dans le papier, par exemple le nombre de caractères différents entre deux versions pour les articles wikipedia.

### Frontend :

Lit les fichiers `.curves` et les affiche : c'est clairement pas une priorité, on peut pour l'instant réutiliser celui des chercheurs originaux.

Peu importe le langage / framework.

## To Do :

- Backend :
    - [ ] création des matrices de distance
    - [ ] déserialisation json
    - [ ] projection des points (MDS)
    - [ ] génération des courbes
    - [ ] export .curve (????)
    - [ ] export vega (https://vega.github.io/)
    - [ ] export tikz (https://tikz.net/)
    - [ ] export tulip (https://tulip.labri.fr/site/)

- [ ] Binding python

## Ressources :

- Serde pour la sérialisation / déserialisation des données (https://serde.rs/), pour le json : https://github.com/serde-rs/json
- CLAP pour créer un programme en ligne de commande (https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
