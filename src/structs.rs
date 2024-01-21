// Run: cargo run --bin rust-structs -q
// Run in watch mode: cargo watch -x "run --bin rust-structs -q"

fn main() {
    println!("Let's define custom types with Rust Structs");

    // Struct (Structure) are custom type that group multiple related data together

    // when structs are useful
    // Enums: choice between a finite set of values
    // Structs: same attributes but different values of a type. e.g. a soccer player, an employee, a car, etc.
        

    // syntax for defining structs

    // model a soccer player that plays for a club and a national team and has a position
    // position is an enum
    // implement DEBUG trait to print the struct

    struct SoccerPlayer {
        name: String,
        club: String,
        position: Position,
        country: String,
    }

    #[derive(Debug)]
    enum Position {
        Goalkeeper,
        Defender,
        Midfielder,
        Forward,
    }

    // syntax for using structs

    let messi = SoccerPlayer {
        name: String::from("Lionel Messi"),
        club: String::from("FC Barcelona"),
        position: Position::Forward,
        country: String::from("Argentina"),
    };

    println!(
        "{} plays for {} as a {:?}",
        messi.name, messi.club, messi.position
    );

    // Tuple and unit structs
    
    // Tuple structs
    // Tuple structs have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of the fields.
    struct Triangle(u32, u32, u32);

    let triangle = Triangle(3, 4, 5);
    println!("Triangle: {}", triangle.0);

    struct Name(String);
    struct Country(String);
    struct Age(u8);

    struct Person(Name, Country, Age);

    let person = Person(
        Name(String::from("John Doe")),
        Country(String::from("Nigeria")),
        Age(21),
    );
    println!("person: {}", (person.0).0);

    // Unit structs
    // Unit structs are most commonly used as marker or sentinel to denote that a particular condition exists or to provide a convenient function signature.    
    // they can be used to define methods for enums

    struct UnitStruct;

    let unit_struct = UnitStruct;


    // Enum variants that look like structs

    // enum variants can have structs as their data
    enum Clock {
        Sundial { hours: u8 },
        Digital { hours: u8, minutes: u8 },
        Analog { hours: u8, minutes: u8, seconds: u8 },
    }
    let clock = Clock::Analog {
        hours: 10,
        minutes: 25,
        seconds: 30,
    };

    

}
