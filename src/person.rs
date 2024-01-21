pub mod person {

    pub struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        pub fn new(name: String, age: u8) -> Self {
            Self { name, age }
        }
        pub fn say_hello(&self) {
            println!("Hello, my name is {} and I am {}.", self.name, self.age)
        }

        pub fn say_hello_to(&self, name: &str) {
            println!(
                "Hello, {}!, my name is {} and I am {}.",
                name, self.name, self.age
            )
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn age_next_year(&mut self) {
            self.age += 1;
        }

        pub fn update_age(&mut self, age: u8) {
            self.age = age;
        }

        pub fn update_name(&mut self, name: String) {
            self.name = name;
        }
        /// make it public
        pub fn display(&self) {
            println!("Name: {}, Age: {}", self.name, self.age);
        }
    }
}