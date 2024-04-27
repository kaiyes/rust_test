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
        println!("{TOKEN_MINT_ADDRESS}");
        if n >= 10 {
            break;
        }
    }
}
