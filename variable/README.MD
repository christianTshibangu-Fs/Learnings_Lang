
**SYNTAXE RUST**

**SYNTAXE VARIABLE**

- variable est immunatable par défaut (non modifiable)
    ex: let x = 4
        x = 6 // erreur E0384  **rustc --explain E0384**
    
- variable peut devenir mutable (modifiable), si on ajoute **mut** devant **let**
    ex: let **mut** x = 4
        x = 6 // 
    
    ex : let mut space = "     ";
        **space = space.len()**; cela déclenche une erreur le type est char et on lui donne un autre type 
    
- variable constate, non seulement par défaut elle est immutable mais elle demeure ainsi. Déclaration:  "**const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;**"

**TYPE DE VARIABLE**

- let entier: u8 = 12; variable entier, declarer avec u/i 8,16,32,64.
- let flotte: f32 = 3.9;
- let z: char = 'Z';
- let nom = "christian";
- let nom: &str = "marie";
- let faux: bool = false;

-Tuple : est une manière générale de regrouper un certain nombre de valeurs avec un **variété de types**; type composé.
    - let tup: (i32, f64, u8) = (500, 6.4, 1); manipuler let (x, y, z) = tup;
    - let tup = (500, 6.4, 1); manipuler let six_point_four = x.1;

-tableau :  d’avoir toujours un nombre fixe d’éléments
    - let a: [i32; 5] = [1, 2, 3, 4, 5];
    - let a = [3; 5]; ou let a = [3, 3, 3, 3, 3];
    - manipuler : let second = a[1];
