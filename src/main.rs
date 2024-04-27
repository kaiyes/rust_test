fn main() {
    let mut i = 0;
    let _x = loop {
        if i > 2 {
            println!("{i}");
            break i * 2;
        };
        i += 1;
    };
}
