// Run: cargo run --bin rust-methods -q
// Run in watch mode: cargo watch -x "run --bin rust-methods -q"


fn main() {
    println!("Let's learn Rust Methods.
    1. When methods are useful.
    2. Syntax for defining methods.
    3. Syntax for calling methods.
    4. Associated functions.
    5. Methods that read data vs .methods that write data.
    ");

    // Methods are functions that are defined within the context of a struct, enum, or trait object.    
    // Methods are useful when you want to define functionality that is specific to a struct, enum, or trait object.
    #[derive(PartialEq)]
    enum Position {
        Goalkeeper,
        Defender,
        Midfielder,
        Forward,
    }

    struct SoccerPlayer {
        name: String,
        club: String,
        position: Position,
        country: String,
    }
    // implement a method that simulate a soccer player scoring a goal via a free kick and the player either scores or misses
    // use some arbitrary logic to determine if the player scores or misses

    // fn simulate_free_kick(player: &SoccerPlayer) -> bool {
    //     // simulate a free kick
    //     // return true if the player scores and false if the player misses
    //     true        
    // }


    impl SoccerPlayer {
        fn simulate_free_kick(&self) -> bool {
            // simulate a free kick
            // return true if the player scores and false if the player misses
            if self.position == Position::Forward {
                return true;
            } else {
                return false;
            }
        }
    }

    let messi = SoccerPlayer {
        name: String::from("Lionel Messi"),
        club: String::from("FC Barcelona"),
        position: Position::Forward,
        country: String::from("Argentina"),
    };
    let result = messi.simulate_free_kick();

    match result {
        true => println!("{} Scored from a free kick", messi.name),
        false => println!("{} Missed from a free kick", messi.name),
    }

    // associated functions

    // associated functions are functions that are defined within the context of a struct, enum, or trait object
    // defined within the impl block. Unlike methods, they don't take self as a parameter

    impl SoccerPlayer {
        fn new(name: String, club: String, position: Position, country: String) -> Self {
            Self {
                name,
                club,
                position,
                country,
            }
        }
    }

    let ronaldo = SoccerPlayer::new(
        String::from("Christiano Ronaldo"),
        String::from("Man United"),
        Position::Forward,
        String::from("Portugal"),
    );

    ronaldo.simulate_free_kick();
    

}