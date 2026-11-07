fn main() {
    // declaration de variable
    println!("--------------------------DEBUT DECLARATION VARIABLE---------------------");
    let mut x = 5;
    println!("la valeur de x : {x}");
    x = 6;
    println!("la valeur de x : {x}");

    // declaration d'un constate
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("la variable déclarée contante : {THREE_HOURS_IN_SECONDS}");

    // observation sur le contournement de immutable
    let y = 7;
    
    let y =y + 6;
    {
        let y =y + 10;
        println!("la valeur de y in scope : {y}"); //Résultat 23
    }
    println!("la valeur de y ouside scope : {y}"); //Résultat 13

    println!("--------------------------FIN DECLARATION VARIABLE---------------------");

    //TYPE DE DONNEE
    println!("--------------------------DEBUT TYPE DE DONNEE---------------------");

    let guess: i32 = "42".parse().expect("Not a number");
    println!("variable devinée : {guess}");

    let entier : u8 = 12;
    println!("variable entière : {entier} est un entier ");

    let flotte_default = 2.4;
    let flotte: f32 = 3.9;
    println!("variable flotte : {flotte_default}, {flotte}");

    let vrai = true;
    let faux: bool = false;
    println!("variable boolean : {vrai}, {faux}");

    let c ='C';
    let z: char = 'Z';
    let nom = "christian";
    let cat = '😻';
    println!("variable char : {c}, {z}, {cat}, {nom}");

    println!("--------------------------FIN TYPE DE DONNEE---------------------");

    //OPERATION
    println!("--------------------------DEBUT OPERATION ---------------------");
    let sum = 5 + 3;
    let difference = 36.5 - 13.7;
    let product = -3 * 5;
    let quotient = 23 / 7;
    let truncated = -5 / 5;
    let remainder = 43 % 5;

    println!("operation resultat : sum {sum}, difference {difference}, product {product}, 
            quotient {quotient}, truncated {truncated}, remainder {remainder}");

    println!("--------------------------FIN OPERATION ---------------------");

    //Tuple
    println!("--------------------------DEBUT TUPLE ---------------------");
    let tup: (i32, f32, u8) = (15, 4.8, 8);
    let (x, y, z) = tup;
    println!("manipuler le Tuple; recupération de la valeur {y} dans le tuple methode 1");

    let tup = (500, 6.4, 1);
    let f = tup.1;

    println!("manipuler le Tuple; recupération de la valeur {f} dans le tuple methode 2");

    println!("--------------------------FIN TUPLE ---------------------");
    
    //Tableau
    println!("--------------------------DEBUT TABLEAU ---------------------");
    let tab =[1,3,18,20,-10,7,];
    let tab1 =tab[4];

    let tab_unite =["RR","GR","Sniper"];
    let tab2 = tab_unite[2];
    println!("acceder au tableau {tab1}, {tab2}" );

}
