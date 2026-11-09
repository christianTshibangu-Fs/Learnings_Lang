
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

- **rustc --explain *E0384*** : expliquer le code d'erreur dans le terminal pour deboger 


**SYNTAXE RUST**

- **use**       : pour importer
- **loop**      : pour le boucle
- **let**       : déclarer une variable (non modifiable)
- **let mut**   : déclarer la variable modifiable
-- **const**    : déclarer une variable constante

**SYNTAXE VARIABLE**
- variable est immunatable par défaut (non modifiable)
    ex: let x = 4
- variable peut devenir mutable (modifiable), si on ajoute **mut** devant **let**
    ex: let **mut** x = 4
        x = 6 // 
- variable constate, non seulement par défaut elle est immutable mais elle demeure ainsi. Déclaration:  "**const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;**"

**TYPE DE VARIABLE**

- let entier: u8 = 12; variable entier, declarer avec u/i 8,16,32,64.
- let flotte: f32 = 3.9;
- let z: char = 'Z';
- let nom = "christian";
- let nom: &str = "marie";
- let faux: bool = false;

-**Tuple** : est une manière générale de regrouper un certain nombre de valeurs avec un **variété de types**; type composé.
    - let tup: (i32, f64, u8) = (500, 6.4, 1); manipuler let (x, y, z) = tup;
    - let tup = (500, 6.4, 1); manipuler let six_point_four = x.1;

-**tableau** :  d’avoir toujours un nombre fixe d’éléments
    - let a: [i32; 5] = [1, 2, 3, 4, 5];
    - let a = [3; 5]; ou let a = [3, 3, 3, 3, 3];
    - manipuler : let second = a[1];

**SYNTAXE de fonction en Rust**

- **fn another_fonction(){}** : declarer une fonction
- **another_fonction()** : faire appele à la fonction
- **fn another_fonction(x: u8)**: declarer une fonction avec argument, il faut preciser le type de l'argument
- **fn another_fonction(x:u8, unite:char){}** : déclarer une fonction avec plusieurs arguments
- **fonction enoncer et expression** :
        let y = {
            let y = 3;
            y +2 // la derniere ligne n'a pas point virgule (return)
        };
- **fonction avec valeur de retour**
        fn plus_one(x: i32) -> **i32** {  //i32 type de retour
            x + 1 // valeur retour
        }


**FLUX DE CONTROLE EN RUST**

- **if{} else{}** : déclarer une condition
-  **let number = if condition { 5} else { 6 };** : if dans une instruction declarer let condition = 5
- **loop{}** : declarer un loop
    ex :     loop {
                println!("boucle infinie {}", count);
                count += 1;
                if count == 100{
                        break;
                    }
            }
        - **loop dans une instruction**    let result = loop {
                                                    count += 1;
                                                    if count == 10{
                                                        break;
                                                    }
                                                };
                                            println!("boucle loop {}", count); 

        - **loop avec label** voir (ex : fn loop_label())

- **while**
    ex :    let mut count = 5;
            while count >= 0 {
                println!("boucle while count {count}");
                count -= 1;
            }

- **for**   
    ex :        let tabs = [14, 28, 56, 98, 88, 98, 63, 19];
                for tab in tabs {
                    println!("parcourir le tab avec for {}",tab);
                }

