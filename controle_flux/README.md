
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




                                