fn main() {
    //array
    let countries = ["Korea", "Japan", "USA"];
    println!("{} {}", countries[1], countries[2]);
    let mix_tuple: (&str, i8) = ("Malaysia", 1);
    println!("{} {}", mix_tuple.0, mix_tuple.1);
}
