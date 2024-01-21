// cargo watch -x "run --bin rust-control-flow -q"
fn main() {
    println!("Let's run Rust Control Flow");

    // if / else if / else 
    
    /*
        if expression {
           ...code... will be evaluated if expression is true
        }

        if expression {
           ...code... will be evaluated if expression is true
        } else {
           ...code... will be evaluated if expression is false
        }

        if expression {
           ...code... will be evaluated if expression is true
        } else if expression {
           ...code... will be evaluated if expression is true
        } else {
           ...code... will be evaluated if all expressions are false
        }

   */

  fn discount(day_of_month:u8) {
    let amount = if day_of_month % 2 ==0 {
        50 // no semicolon at the end of the expression
    } else {
        10 // no semicolon at the end of the expression
    }; // semicolon at the end of the expression to assign it to the variable
    println!("you discount amount is : {}", amount);
  }
   discount(2);


    // loop, while, for 
    // loops lets you run your code repeatedly until a condition is met
    
    // loop if true break
    loop {
        println!("Looping forever!");
        let mut secret_word = String::new();
        std::io::stdin().read_line(&mut secret_word).expect("Failed to read line");
        if secret_word.trim() == "rust" {
            println!("You guessed correctly!");
            break;
        }
    }

    // while loop
    /*
      while expression {
        ...code... will be evaluated as long as the expression is true
      }
     */

    let mut secret_word = String::new();
    while secret_word.trim() != "rust" {
        println!("Guess the secret word!");
        secret_word = String::new();
        std::io::stdin().read_line(&mut secret_word).expect("Failed to read line");        
    }
    println!("You guessed correctly!");
    
    // for loop
    // for loop is used to iterate over a collection of items

    /*
      for item in collection {
        ...code... will be evaluated for each item in the collection
      }
     */

    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers {
        println!("number: {}", number);
    }

    let tokens = [1,2,3,4,5,6,7,8,9,10];
    for token in tokens.iter() {
        println!("Now Serving token: {}", token);
    }


    for token in 1..11 {
        println!("Now Serving token: {}", token);
    }

    // match
    // pattern matching is a way to compare a value against a series of patterns and execute code based on which pattern matches
    /*
      match expression {
        pattern => ...code... will be evaluated if the pattern matches
        pattern => ...code... will be evaluated if the pattern matches
        pattern => ...code... will be evaluated if the pattern matches
        ...
        _ => ...code... will be evaluated if none of the patterns match
      }
     */
    let number = 13;
    match number {
        1 => println!("It is one"),
        2 => println!("It is two"),
        3 => println!("It is three"),
        _ => println!("It doesn't match"), // _ is a catch all pattern
    }

    // exhaustive checking - this means that you have to cover every possible case and prevent bugs from creeping into your code

    let is_confirmed = true;
    let is_active = true;

    // if any of the case is not covered, the compiler will throw an error. It will tell exactly which case is not covered
    match (is_confirmed, is_active) {
        (true, true) => println!("Both are true"),
        (true, false) => println!("Confirmed but not active"),
        (false, true) => println!("Not confirmed but active"),
        (false, false) => println!("Neither confirmed nor active"),
    }    
}