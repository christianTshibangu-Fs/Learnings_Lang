fn main() {
    println!("Hello, world!");

    another_fonction();

    fonction_parametre(5);

    fonction_para_multiple("christian", 15 );

    //enoncer et expression
    let y = {
        let y = 3;
        y +2 // la derniere ligne n'a pas point virgule (return)
    };
    println!("expression {y}");

    let x = plus_one(10);
    println!("la valeur de retour de la fonction plus_one est : {}", x);


}

//declarer une fonction
fn another_fonction(){
    println!("another fonction");
}

//declarer une fonction avec parametre
fn fonction_parametre(x: i32){
    println!("fonction avec le parametre {x}");
}

// declarer une fonction avec plusieurs parametre
fn fonction_para_multiple(nom: &str, age: u16 ){
    println!("fonction avec le parametre, le nom est : {nom} et l'age {age} comme parametre");
}

//fonction avec valeur de retour
fn plus_one(x: i32) -> i32 {
    x + 1
}
