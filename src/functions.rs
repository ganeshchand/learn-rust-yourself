// rust functions

fn main() {
  println!("Let's run Rust Functions");
  
  // syntax for defining a function
   
   /* 
   fn name(param1: type1, ...) -> return_type {
     ...body...
   }
    */
    
    fn next_birthday(name: &str, current_age: u8) { // first parameter is a string slice
        let next_age = current_age + 1;
      println!("Hello {}, on your next birthday, you'll be {}", name, next_age);
      // this function doesn't return anything
    }
    


  // syntax for calling a function

    next_birthday("John", 21);
    next_birthday("Jane", 20);
  
  
  // return values from functions

    fn square(num: i32) -> i32 {
      num * num // no semicolon at the end of the expression
      // you can also use the return keyword
    }

    let result = square(2);
    println!("result from square: {}", result);


    // function retuning a tuple

    fn get_name() -> (String, String) {
      ("John".to_string(), "Doe".to_string())
    }

    let (first_name, last_name) = get_name();

    println!("first_name: {}, last_name: {}", first_name, last_name);


    // function returning a function

    fn get_greeter() -> fn(&str) -> String {
      fn greeter(name: &str) -> String {
        format!("Hello, {}!", name)
      }
      greeter
    }

    let greeter = get_greeter();
    let greeting = greeter("John");
    println!("greeting: {}", greeting);


    // function taking a function as a parameter and returning a function

    fn get_greeter_2() -> fn(&str) -> String {
      fn greeter(name: &str) -> String {
        format!("Hello, {}!", name)
      }
      greeter
    }

    fn greet(greeter: fn(&str) -> String, name: &str) -> String {
      greeter(name)
    }

    let greeter = get_greeter_2();
    let greeting = greet(greeter, "John");
    println!("greeting: {}", greeting);
    

}
