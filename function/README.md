
**Utisation de fonction en Rust**

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