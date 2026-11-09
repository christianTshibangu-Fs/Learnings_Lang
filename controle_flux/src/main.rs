fn main() {
    println!("Hello, world!");
    println!("--------------Début if --------");
    branch();
    println!("--------------Fin if --------");

    println!("--------------Début boucle --------");
    loop_example();
    loop_label();
    loop_while();
    loop_for();
    println!("--------------Fin boucle --------");
}

fn branch(){
    let number = 5;

    if number < 10 {
        println!("la condition est vrai");
    }else {
        println!("la condition es fause");
    }

    let number = 12;

    if number % 4 == 0 {
        println!("nombre {} est divisible par 4 ", number);
    }else if number % 3 == 0 {
        println!("nombre {} est divisible par 3", number);
    }else if number % 2 == 0 {
        println!("nombre {} est divisible par 2", number);
    }else {
        println!("nombre {} n'est divisible par 4, 3, ni 2", number);
    }
    branch_in_instruction();
}

fn branch_in_instruction(){
    let condition = true;
    let number = if condition { 5} else { 6 };
    println!("le nombre dans la variable d'instruction est : {}", number);

    let condition = 'A';
    let resultat = if condition == 'G'{
            85
        } else if condition == 'D'{
            75
        }else if condition == 's'{
            65
        } else if condition == 'A'{
            55
        }else{
            45
        };
        println!("le resultat est de : {},  l'etudiant a reçu une note de {}" , resultat  , condition);

}

fn loop_example(){

    let mut count = 0;
    let result = loop {
            count += 1;
            if count == 10{
                break;
            }
        };
    println!("boucle loop {}", count);

    loop {
        println!("boucle infinie {}", count);
        count += 1;
        if count == 100{
                break;
            }
    }
}

fn loop_label(){
    let mut count = 0;
    'counting_loop : loop {
        println! ("loop label count {}", count);
        let mut remaining = 10;
        loop {
            println!("loop remaining {}",remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn loop_while(){
    let mut count = 5;
    while count >= 0 {
        println!("boucle while count {count}");
        count -= 1;
    }

    let tab = [14, 28, 56, 98, 88, 98, 63, 19];
    let mut index = 0;
    while index < tab.len() {
        println!("print Tab avec while : {}", tab[index]);
        index += 1;
    }
}

fn loop_for(){
    let tabs = [14, 28, 56, 98, 88, 98, 63, 19];
    for tab in tabs {
        println!("parcourir le tab avec for {}",tab);
    }
    for number in (1..5).rev(){
        println!("for avec rev {}", number);
    }
}
