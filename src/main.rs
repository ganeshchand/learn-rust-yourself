// mod person;
// use person::Person;
fn main() {
    println!("Hello, world!");

    fn multiply_numbers(a: i32, b: Option<i32>) -> i32 {
        // let result = if let Some(b) = b { a * b } else { a };
        // return result;
        match b {
            Some(b) => a * b,
            None => a,        
        }
    }

    println!("multiply_numbers(2, None): {}", multiply_numbers(2, None));
    println!("multiply_numbers(2, Some(4)): {}", multiply_numbers(2, Some(4)));

    // let mut person = Person::new("Alice".to_string(), 32);
    // person.display();
}
