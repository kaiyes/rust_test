use std::i8;

fn main() {
    let _num: i8 = 11; //i32
    println!("the number is {}", _num);
    let name: &str = "kaiyes";
    let middle_name: &str = "ahmad";
    let sur_name: &str = "ansary";
    //this prints name
    /*
     * multi
     * line
     * comments
     */
    println!("my name is {} {} {}", name, middle_name, sur_name);
    let mut _num: i16 = 210; // making it mutable
    println!("mutated number is: {}", _num);
}
