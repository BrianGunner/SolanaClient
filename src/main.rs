
use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction;
use std::str::FromStr;

fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);
    let sender = read_keypair_file("id.json").map_err(|e|anyhow::anyhow!("could not load id.json:{}",e))?;
    let sender_pubkey = sender.pubkey();
    println!("{}",sender_pubkey);
    Ok(())
    
}
