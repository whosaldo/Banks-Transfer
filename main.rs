// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
#![allow(dead_code)]
#![allow(unused_must_use)]


use std::io;
use std::collections::HashMap;
/////////////////////////
fn get_text() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input.trim().to_string()
}
///////////////////////////
fn add_bills() -> (String, String)
{
    println!("introduceti numele de la cine ati imprumutat");
        let name = get_text();
         
        println!("Nume introdus: {:?}, introduceti acum suma", name);
        let suma = get_text();

        println!("Nume introdus: {:?}, Suma: {:?}", name, suma);
        
        (name, suma)
        
}
//////////////////////////////
fn main() 
{

    println!(" Compiled succesfuly!\n\n Introduceti ce doriti sa faceti: \n 1: Add Bills \n 2: View existing bills \n 3: Sterge Tranzac \n 4: Editeaza tranzactii");
    
     let mut input = get_text();
     let mut manager: HashMap<String, String> = HashMap::new();
     //exemple
     manager.insert("ALdo".to_owned(), "200".to_owned());
     manager.insert("Marin".to_owned(), "560".to_owned());
     manager.insert("Iohannis".to_owned(), "450".to_owned());
     manager.insert("Florel".to_owned(), "850".to_owned());
    loop 
    {
        if input == "1"
        {
       /////////////////////////////////////////////////
        let (nume, numar) = add_bills();
        manager.insert(nume, numar);
        println!("mai doriti sa faceti o tranzactie? (scrie 'y' daca da)");
        let input2 = get_text();
        if input2 == "y"{
        continue;
        }else{
            input = "2".to_owned();}
        }
        /////////////////////////////////////////////
        else if input == "2"
        {
            for (nume, suma) in &manager
        {
            println!("Nume: {:?} -> {:?} lei", nume, suma);
        }
          break;
        }
        /////////////////////////////////////////////
        else if input == "3"
        {
            println!("Choose a current transaction to remove:");
            for (nume, suma) in &manager
        {
            println!("Nume: {:?} -> {:?} lei", nume, suma);
        }
            println!("introduceti numele care vreti sa stergeti:");
             let alegere = get_text();
         
            manager.remove(&alegere);
            println!("tranzactia s-a sters");
            println!("\ntranzactile de acum:");
            for (nume, suma) in &manager
        {
            println!("Nume: {:?} -> {:?} lei", nume, suma);
        }
            break;
        }
        ////////////////////////////////////////////////
        else if input == "4"
        {
              println!("Tranzactii:");
              for (nume, suma) in &manager
              {
              println!("Nume: {:?} -> {:?} lei", nume, suma);
              }
                 
              println!("Doriti sa modifcati valoarea sau numele? Introduceti 'valoare' sau 'nume' ");
              let alegere = get_text();

              if alegere == "valoare"
              {
                println!("introduceti numele la care doriti sa schimbati valoarea:");
                let  cuvant_magic = get_text();
                println!("introduceti cu cat doriti sa schimbati valoarea:");
                let  valoare_noua = get_text();
                
                  manager.remove(&cuvant_magic);
                  manager.insert(cuvant_magic, valoare_noua);
   
                for (nume, suma) in &manager
                {
                    println!("Nume: {:?} -> {:?} lei", nume, suma);
                }
                  break;
              }
              else if alegere == "nume"
              {
                println!("introduceti numele la care doriti sa schimbati numele:");
                let  cuvant_magic = get_text();
                println!("introduceti numele cu care vreti sa il schimbati:");
                let  nume_nou = get_text();
                println!("introduceti valoarea cu care vreti sa il introduceti:");
                let valoare_noua = get_text();
                
                manager.remove(&cuvant_magic);
                manager.insert(nume_nou, valoare_noua);

                for (nume, suma) in &manager
                {
                    println!("Nume: {:?} -> {:?} lei", nume, suma);
                }
                  break;
              }

        }
}
}
