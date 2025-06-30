use std::io::*;
use rand::prelude::*;
fn main(){
    let guess_list:[&str;4] = ["grapes","mango","apple","orange"];

    let mut rng = rand::rng();

    let i = rng.random_range(0..guess_list.len());
    let random_fruit:&str= guess_list[i];
    //this is just for testing , then this below line must be commented 
    //println!("Random fruit is {}",random_fruit);
    //take input from user and do error handling using match
    println!("Enter the fruit name here");
    let mut input = String :: new();
    loop {
        input.clear();
        match stdin().read_line(&mut input){
            Ok(_)=>{
                let fruit_selected = input.trim().to_lowercase();
                if !guess_list.contains(&fruit_selected.as_str()){
                    println!("Fruit entered was not found");
                    continue;
                }
                else {
                    if fruit_selected == random_fruit{
                        println!("You guessed it , you can become a professional gambler");
                        break;
                    }
                    else {
                        println!("You guessed wrong, better luck next time");
                    }
                }
            }
            Err(error)=>{
                println!("Some error while taking input: {}",error);
            }
        }
    }
}