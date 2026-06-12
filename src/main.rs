
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction;
use std::str::FromStr;



fn main()->anyhow::Result<()>{
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let sender = read_keypair_file("id.json").expect("Failed to read Keypair");
    let sender_pubkey = sender.pubkey();
    let sender_secret = sender.secret();

    let receiver = Keypair::new();
    let receiver_pubkey = receiver.pubkey();

    let sender_balance = client.get_balance(&sender_pubkey)?;
   
    let receiver_balance = client.get_balance(&receiver_pubkey)?;
    let blockhash = client.get_latest_blockhash()?;
    
    let transfer_instruction = system_instruction::transfer(
        &sender_pubkey, 
        &receiver_pubkey, 
        900_300_234,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction], 
        Some(&sender_pubkey), 
        &[&sender], 
        blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction Signature : {}",signature);
    let sender_balance = client.get_balance(&sender_pubkey)?;
    let receiver_balance = client.get_balance(&receiver_pubkey)?;
    println!("Updated Sender balance: {}, Updated receiver balance: {}",sender_balance as f64 / 1_000_000_000.0,receiver_balance as f64 / 1_000_000_000.0);


    Ok(())

}