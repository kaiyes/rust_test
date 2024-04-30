use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]

struct Account {
    public_key: String,
    private_key: String,
    lets_eat: String,
}

fn main() -> Result<()> {
    let mut file = File::open("accounts.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let accounts: Vec<Account> = serde_json::from_str(&contents)?;

    // Specify the Solana cluster you want to connect to
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");

    for account in accounts {
        let pubkey = Pubkey::from_str(&account.public_key)?;
        let balance: f64 = client.get_balance(&pubkey)? as f64;

        println!(
            "Public Key: {}, Balance: {}",
            account.public_key,
            balance / LAMPORTS_PER_SOL
        );
    }

    Ok(())
}

const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;
