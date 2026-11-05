
I. Exécution basic 

Initiation au language de programmation Rust

- implementation et éxecution basic
- creer un repertoire et un fichier 'main.rs' en général
- compiler le programme avec la commande **'rustc nom du fichier'** ex: rustc main.rs
- éxécuter le programme avec la commande **'.\main'**

- **rustup doc --book** documentation rust en local

II. Exécution avec Cargo

CARGO le gestionnaire du language Rust (comme pypi en python)

- créer un projet avec la commande **'cargo new nom_projet'**
- initialiser : 
    - fichier Cargo.toml(Tom’s Obvious, Minimal Language) qui est le format de configuration de Cargo,
    - dossier src pour stocker le code source du projet
    - un fichier main.rs dans le dossier src qui initialiser le projet avec un code minumun
    - un git initial
    - un fichier .gitignores

- **cargo init**, dans la partie I qui exclut cargo dans l'éxécution du projet, il serai preferable        
    de vouloir revenir sur la norme Cargo, alors :
    - créer un dossier src et le fichier main.rs à l'interieure
    - créer le fichier Cargo.toml à la racine du projet
    - éxécuter la commande 'cargo init'

**COMMANDE CARGO ESSENTIEL**

- **cargo build** : compiler le programme en se situant sur le dossier du projet, 
    - il crée un exe dans target/debug/nom_du_projet.exe
    - et éxécuter : .\target\debug\nom_du_projet.exe

- **cargo run** : compile et éxécute le projet

- **cargo check** : verifie que le programme compile sans pour autant créer un éxécutable

- **cargo build --release** : compile avec des optimisations et construit pour le version final en production


**SYNTAXE RUST**

- **use** : pour importer
-**loop** : pour le boucle



