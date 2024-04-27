const RPC_ADDRESS: &str = "quickNode";
const TOKEN_MINT_ADDRESS: &str = "whale token mint address";

fn main() {
    println!("{}", RPC_ADDRESS);
    other()
}

fn other() {
    let x = 15;
    if x > 10 {
        println!("{}", TOKEN_MINT_ADDRESS);
    } else {
        println!("{}", RPC_ADDRESS)
    }
}
