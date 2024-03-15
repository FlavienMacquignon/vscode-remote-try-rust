/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

 use std::io;
 use rand::thread_rng;


 fn main() {    
      println!("Guess the number!");
 
     let secret_number = rand::thread_rng().gen_range(1..=100);
 
     println!("The secret number is: {secret_number}");
 
     println!("Please input your guess.");
 
     let mut guess = String::new();
 
     io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
   println!("You guessed: {guess}");
 }
