fn main() {
    let number = 13;

    match number {
        // Match a single value
        10 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("rutgart"),
        _ => println!("special"),
    }
}
