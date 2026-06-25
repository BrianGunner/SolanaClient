use anyhow::{Context, Error, Ok, Result};
use solana_client::pubsub_client::PubsubSignatureClientSubscription;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction::{self, SystemInstruction};
use spl_token::state::Account as TokenAccount;
use solana_program::program_pack::Pack;
use std::any;
use std::str::FromStr;
use spl_token::state::Mint;
use std::fs;
use solana_sdk::signature::write_keypair_file;




fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);

    let main_account = read_keypair_file("id.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let main_pubkey = main_account.pubkey();
    println!("Main Account Pubkey: {}",main_pubkey);

    let solomon_token = read_keypair_file("solomon-mint.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let solomon_pubkey = solomon_token.pubkey();
    println!("Solomon Token Mint Account Pubkey: {}",solomon_pubkey);

    let token_account = read_keypair_file("token-account.json").map_err(|e|anyhow::anyhow!("could not load token-account: {}",e))?;
    let token_pubkey = token_account.pubkey();
    println!("Solomon Token Account Pubkey: {}",token_pubkey);

    let recent_blockhash = client.get_latest_blockhash()?;

    let mint_to_instruction = spl_token::instruction::mint_to(
        &spl_token::ID, 
        &solomon_pubkey, 
        &token_pubkey, 
        &main_pubkey, 
        &[], 
        1_000_000_000_000,
    )?;

    let mint_to_tx = Transaction::new_signed_with_payer(
        &[mint_to_instruction], 
        Some(&main_pubkey), 
        &[&main_account], 
        recent_blockhash
    );

    let signature = client.send_and_confirm_transaction(&mint_to_tx);

    match signature{
        std::result::Result::Ok(sig)=>println!("Solomon Tokens minted into token account with signature: {}",sig),
        Err(msg)=>println!("{}",msg),
    }

    
    Ok(())
}