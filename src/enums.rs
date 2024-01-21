// Run: cargo run --bin rust-enums -q
// Run in watch mode: cargo watch -x "run --bin rust-enums -q"

fn main() {
    println!("Let's define custom types with Rust Enums");

    // Enums are a way to define a custom type in Rust by enumerating its finite set of possible values
    // They work with pattern matching to help you write safer code
    // Enums are useful when you have a finite set of values that you know about at compile time

    // properties of enums
    // 1. Can only be one value at a time
    // 2. Can list (enumerate) all possible values

    // define an enum

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn move_to_direction(direction: Direction) {
        match direction {
            Direction::Up => println!("Moving Up"),
            Direction::Down => println!("Moving Down"),
            Direction::Left => println!("Moving Left"),
            Direction::Right => println!("Moving Right"),
        }
    }

    move_to_direction(Direction::Up);

    // define an enum with values

    enum Clock {
        Sundial(u8),        // only hour
        Digital(u8, u8),    // only hour and minute
        Analog(u8, u8, u8), // hour, minute, second
    }

    fn tell_time(clock: Clock) {
        match clock {
            Clock::Sundial(hour) => println!("It is about {} o'clock", hour),
            Clock::Digital(hour, minute) => {
                println!("It is {} minutes past {} o'clock", minute, hour)
            }
            Clock::Analog(hour, minute, second) => println!(
                "It is {} minutes and {} seconds past {} o'clock",
                minute, second, hour
            ),
        }
    }

    tell_time(Clock::Sundial(10));
    tell_time(Clock::Digital(10, 25));
    tell_time(Clock::Analog(10, 25, 30));

}
