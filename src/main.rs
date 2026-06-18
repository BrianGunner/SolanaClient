use anyhow::{Context, Ok, Result};
use solana_client::pubsub_client::PubsubSignatureClientSubscription;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction::{self, SystemInstruction};
use std::any;
use std::str::FromStr;

fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);
    let sender = read_keypair_file("id.json").map_err(|e|anyhow::anyhow!("could not load id.json{}",e))?;
    let sender_pubkey = sender.pubkey();
    let new_account = Keypair::new();
    let new_pubkey = new_account.pubkey();
    println!("New Pubkey: {}",new_pubkey);
    let space = 64 as usize;
    let lamports = client.get_minimum_balance_for_rent_exemption(space)?;
    let latest_blockhash = client.get_latest_blockhash()?;

    let instruction_create_account = system_instruction::create_account(
        &sender_pubkey, 
        &new_pubkey, 
        lamports, 
        space as u64, 
        &solana_sdk::system_program::id(),
    );

    let transaction_create_account = Transaction::new_signed_with_payer(
        &[instruction_create_account], 
        Some(&sender_pubkey), 
        &[&sender,&new_account], 
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction_create_account)?;
    println!("Account created with Signature: {}",signature);

    let new_account_details = client.get_account(&new_pubkey)?;
    println!("New account lamports: {}",new_account_details.lamports);
    println!("New account owner: {}",new_account_details.owner);
    println!("New account data: {:?}",new_account_details.data);
    println!("New account executable: {}",new_account_details.executable);

    Ok(())
}