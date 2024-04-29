use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]

struct Account {
    public_key: String,
    private_key: String,
}

fn main() -> Result<()> {
    let mut file = File::open("accounts.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let accounts: Vec<Account> = serde_json::from_str(&contents)?;

    for account in accounts {
        println!("Public Key: {}", account.public_key);
    }

    Ok(())
}
