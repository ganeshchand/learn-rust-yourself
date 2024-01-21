fn main() { 
    // boolean type
    let a = true;
    let b = false;

    if a {
        println!("a is true");
    }

    if b {
        println!("b is true");
    } else {
        println!("b is false");
    }

    match b {
        true => println!("b is true"),
        false => println!("b is false"),
    }

    // integer types

    // signed integers   (i8, i16, i32, i64, i128, isize)
    // unsigned integers (u8, u16, u32, u64, u128, usize)

    let a: i8 = 10; // 8 bits : first bit is sign bit (0 = positive, 1 = negative), remaining 7 bits are for the number
    let b: i16 = 10; // 16 bits : first bit is sign bit (0 = positive, 1 = negative), remaining 15 bits are for the number

    let c: u8 = 10; // 8 bits : all 8 bits are for the number

    // isize and usize are dependent on the architecture of the system
    // 64 bit architecture: isize = i64, usize = u64
    // 32 bit architecture: isize = i32, usize = u32
    // you use isize and usize when you want to use the maximum number of bits available on the system
    // typical use case: indexing into a collection, like an array or vector or counting something

    let a = [100, 200, 300];
    let first_element = a[0]; // 100 - here the index 0 is of type usize because it is an array.
    
    // by default, if you don't specify the type of an integer without a decimal point, it is assumed to be i32
    let a = 10; // i32
    
    // by default, decimal numbers are f64
    let a = 10.2; // f64


    // floating point types - numbers with decimal points

    // f32 and f64

    let a = 10.2; // f64 - default
    let c: f32 = 10.2; // f32

    // char type - single unicode character - they are a 32 bit unicode scalar value, not just ASCII
    // use single quotes for char type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}",heart_eyed_cat);

    // Tuples
    // tuples are fixed length, immutable lists of values
    // tuples can contain values of different types
    // tuples are useful when you want to express a compound value and you know how many elements you want to combine
    // tuples are useful when you want to return multiple values from a function
    // tuples are useful when you want to pass multiple values to a function

    let tuple = (1, "john", "john@gmail.com", 21, true);

    // accessing tuple elements
    let id = tuple.0;
    let name = tuple.1;
    let email = tuple.2;
    let age = tuple.3;
    let is_married = tuple.4;

    println!("id: {}, name: {}, email: {}, age: {}, is_married: {}", id, name, email, age, is_married);

    // destructuring a tuple
    let (id, name, email, age, is_married) = tuple;

    println!("id: {}, name: {}, email: {}, age: {}, is_married: {}", id, name, email, age, is_married);

   // Arrays

    // arrays are fixed length, immutable lists of values
    // arrays can contain values of the same type
    // arrays are useful when you want to express a compound value and you know how many elements you want to combine
    // arrays are useful when you want to return multiple values from a function
    // arrays are useful when you want to pass multiple values to a function

    let array = [1, 2, 3, 4, 5];

    // accessing array elements
    let first_element = array[0];
    let second_element = array[1];
    println!("first_element: {}, second_element: {}", first_element, second_element);

    // destructuring an array
    let [first_element, second_element, third_element, fourth_element, fifth_element] = array;
    println!("first_element: {}, second_element: {}, third_element: {}, fourth_element: {}, fifth_element: {}", first_element, second_element, third_element, fourth_element, fifth_element);

    // size of an array
    let size_of_array = array.len();
    println!("size_of_array: {}", size_of_array);

   // mutable array
   let mut array = [1, 2, 3, 4, 5];
   println!("before: array: {:?}", array);
    array[0] = 100;
    println!("after array: {:?}", array); // {:?} is used to print an array. it is called a debug print. It will be
    // replaced by a string representation of the array. This is made possible by the Debug trait.
    // Any type that implements the Debug trait can be printed using {:?}.
    // You can derive the Debug trait for your custom types by adding #[derive(Debug)] before the type definition.  
    
    // you cannot add or remove elements from an array. i.e. the length of an array is fixed.
    // you can only update the values of an array.

    // if you want to add or remove elements from a collection, use a vector, part of the standard library

    // Vectors
    let mut vector = vec![1, 2, 3, 4, 5];
    // add an element to the end of the vector
    vector.push(6);
    println!("vector: {:?}", vector);

    // Slices

    // slices are a reference to a contiguous sequence of elements in a collection
    // slices are useful when you want to reference a part of a collection

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[0..3]; // slice is a reference to the first 3 elements of the array
    println!("slice: {:?}", slice); // 0..3 is a range, inclusive of the first index, exclusive of the last index
    
    // string slices
    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    println!("string slice: {}", hello);
}