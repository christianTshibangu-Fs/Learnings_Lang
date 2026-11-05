
CARGO le gestionnaire du language Rust (comme pypi en python)

- créer un projet avec la commande **'cargo new nom_projet'**
- initialiser : 
    - fichier Cargo.toml(Tom’s Obvious, Minimal Language) qui est le format de configuration de Cargo,
    - dossier src pour stocker le code source du projet
    - un fichier main.rs dans le dossier src qui initialiser le projet avec un code minumun
    - un git initial
    - un fichier .gitignores

**COMMANDE CARGO ESSENTIEL**

- **cargo build** : compiler le programme en se situant sur le dossier du projet, 
    - il crée un exe dans target/debug/nom_du_projet.exe
    - et éxécuter : .\target\debug\nom_du_projet.exe

- **cargo run** : compile et éxécute le projet

- **cargo check** : verifie que le programme compile sans pour autant créer un éxécutable

- **cargo build --release** : compile avec des optimisations et construit pour le version final en production




