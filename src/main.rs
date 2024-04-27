const RPC_ADDRESS: &str = "quickNode";
const TOKEN_MINT_ADDRESS: &str = "whale token mint address";

fn main() {
    println!("{RPC_ADDRESS}");
    other()
}

fn other() {
    let mut n = 0;
    loop {
        n += 1;
        if n == 7 {
            println!("{TOKEN_MINT_ADDRESS}");
            continue;
        }
        if n > 10 {
            break;
        }
        println!("{n}");
    }
}
