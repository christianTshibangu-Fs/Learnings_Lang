use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    println!("Deviner le nombre!");
    let secret_number = rand::rng().random_range(1..=100);
    // println!("le nombre secret est : {secret_number} ");
    println!("inserer le nombre entre 1 à 100");
    loop {
        println!("Veuillez entrer votre devinette.");
        let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Erreur de lecture de la ligne");
    //    let guess: u32 = guess.trim().parse().expect("veuillez taper un nombre!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Vous avez deviné: {guess}", );

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Trop petit!"),
            Ordering::Greater => println!("Trop grand!"),
            Ordering::Equal => {
                println!("Vous avez gagné!");
                break;
            }
        }
    }
    
}