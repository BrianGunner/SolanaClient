use anyhow::{Context, Ok, Result};
use solana_client::pubsub_client::PubsubSignatureClientSubscription;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction::{self, SystemInstruction};
use std::str::FromStr;

fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);
    let sender = read_keypair_file("id.json").map_err(|e|anyhow::anyhow!("could not load id.json{}",e))?;
    let sender_pubkey = sender.pubkey();
    let receiver_1_pubkey = Pubkey::from_str("BBbMGNN1d4mmbEwxvmyTb82QHB6fbtemh2CQMBCtZHKo")?;
    let reveiver_2_pubkey = Pubkey::from_str("CMo1gA6YQebnSxXNYK8KawpczFaYLuUgyAf5FRAoryRQ")?;
    let transfer_instruction_1 = system_instruction::transfer(&sender_pubkey, &receiver_1_pubkey, 1_000_000);
    let transfer_instruction_2 = system_instruction::transfer(&sender_pubkey, &reveiver_2_pubkey, 1_000_000);
    let latest_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction_1,transfer_instruction_2], 
        Some(&sender_pubkey), 
        &[&sender], 
        latest_blockhash,
    );
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction Signature: {}",signature);
    let sender_balance = client.get_balance(&sender_pubkey)?;
    let receiver_1_balance = client.get_balance(&receiver_1_pubkey)?;
    let receiver_2_balance = client.get_balance(&reveiver_2_pubkey)?;
    println!("Sender Balance: {}, Receiver 1 Balance: {}, Receiver 2 Balance: {}",sender_balance,receiver_1_balance,receiver_2_balance);
    Ok(())
}